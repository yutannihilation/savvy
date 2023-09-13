use quote::format_ident;
use syn::parse_quote;

use crate::unextendr_fn::{UnextendrFn, UnextendrFnType};
use crate::utils::extract_docs;

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

#[cfg(test)]
mod tests {
    use super::UnextendrFnType::*;
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_impl() {
        let item_impl: syn::ItemImpl = parse_quote!(
            #[unextendr]
            impl Person {
                fn new() -> Self {
                    Self {
                        name: "".to_string(),
                    }
                }

                fn set_name(&mut self, name: StringSxp) {
                    self.name = name.iter().next().unwrap().to_string();
                }

                fn name(&self) -> unextendr::Result<unextendr::SEXP> {
                    let mut out = OwnedStringSxp::new(1);
                    out.set_elt(0, self.name.as_str());
                    Ok(out.into())
                }

                fn do_nothing() {}
            }
        );

        let parsed = UnextendrImpl::new(&item_impl);
        assert_eq!(parsed.ty.to_string().as_str(), "Person");

        assert_eq!(parsed.fns.len(), 4);

        assert_eq!(parsed.fns[0].fn_name.to_string().as_str(), "new");
        assert!(matches!(parsed.fns[0].fn_type, Constructor(_)));

        assert_eq!(parsed.fns[1].fn_name.to_string().as_str(), "set_name");
        assert!(matches!(parsed.fns[1].fn_type, Method(_)));

        assert_eq!(parsed.fns[2].fn_name.to_string().as_str(), "name");
        assert!(matches!(parsed.fns[2].fn_type, Method(_)));

        assert_eq!(parsed.fns[3].fn_name.to_string().as_str(), "do_nothing");
        assert!(matches!(parsed.fns[3].fn_type, AssociatedFunction(_)));
    }
}
