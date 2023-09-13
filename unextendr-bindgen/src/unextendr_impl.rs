use quote::format_ident;
use syn::parse_quote;

use crate::extract_docs;
use crate::UnextendrFn;
use crate::UnextendrFnType;

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
            .iter()
            .filter_map(|f| match f {
                syn::ImplItem::Fn(impl_item_fn) => {
                    let ty = orig.self_ty.as_ref().clone();

                    let fn_type = match (is_method(impl_item_fn), is_ctor(impl_item_fn)) {
                        (true, false) => UnextendrFnType::Method(ty),
                        (false, true) => UnextendrFnType::Constructor(ty),
                        (false, false) => UnextendrFnType::AssociatedFunction(ty),
                        (true, true) => panic!("Should not happen"),
                    };

                    Some(UnextendrFn::from_impl_fn(impl_item_fn, fn_type))
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

// check if the first argument is `self`
fn is_method(impl_item_fn: &syn::ImplItemFn) -> bool {
    matches!(
        impl_item_fn.sig.inputs.first(),
        Some(syn::FnArg::Receiver(_))
    )
}

// check if the return type is `Self`
fn is_ctor(impl_item_fn: &syn::ImplItemFn) -> bool {
    match &impl_item_fn.sig.output {
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            syn::Type::Path(type_path) => {
                type_path
                    .path
                    .segments
                    .last()
                    .expect("Unexpected path")
                    .ident
                    .to_string()
                    .as_str()
                    == "Self"
            }
            _ => false,
        },
        _ => false,
    }
}
