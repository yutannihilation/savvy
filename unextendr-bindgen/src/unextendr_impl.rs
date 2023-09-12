use quote::format_ident;
use syn::parse_quote;

use crate::extract_docs;
use crate::UnextendrFn;

pub struct UnextendrImpl {
    /// Doc comments
    pub docs: Vec<String>,
    /// Attributes except for `#[unextendr]`
    pub attrs: Vec<syn::Attribute>,
    /// Original type name
    pub ty: syn::Ident,
    /// Methods and accociated functions
    pub fns: Vec<UnextendrFn>,
    /// Original body of the impl
    pub orig: syn::ItemImpl,
}

impl UnextendrImpl {
    pub fn new(orig: &syn::ItemImpl) -> Self {
        let mut attrs = orig.attrs.clone();
        // Remove #[unextendr]
        attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));
        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());

        let ty = match orig.self_ty.as_ref() {
            syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
            _ => {
                // TODO: propagate syn::Error
                // panic!("should not happen");
                format_ident!("UNEXPETED")
            }
        };

        let fns: Vec<UnextendrFn> = orig
            .items
            .clone()
            .iter_mut()
            .filter_map(|f| match f {
                syn::ImplItem::Fn(impl_item_fn) => {
                    Some(UnextendrFn::from_impl_fn(impl_item_fn, &orig.self_ty))
                }
                _ => None,
            })
            .collect();

        Self {
            docs,
            attrs,
            ty,
            fns,
            orig: orig.clone(),
        }
    }

    pub fn make_inner_fns(&self) -> Vec<syn::ItemFn> {
        self.fns.iter().map(|f| f.make_inner_fn()).collect()
    }

    pub fn make_outer_fns(&self) -> Vec<syn::ItemFn> {
        self.fns.iter().map(|f| f.make_outer_fn()).collect()
    }
}
