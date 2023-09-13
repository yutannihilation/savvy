mod unextendr_fn;
mod unextendr_impl;
use syn::Attribute;

pub use unextendr_fn::{
    make_c_header_file, make_c_impl_file, make_r_impl_file, parse_unextendr_fn, UnextendrFn,
    UnextendrFnArg, UnextendrFnType,
};

pub use unextendr_impl::UnextendrImpl;

pub fn extract_docs(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            match &attr.meta {
                syn::Meta::NameValue(nv) => {
                    // Doc omments are transformed into the form of `#[doc =
                    // r"comment"]` before macros are expanded.
                    // cf., https://docs.rs/syn/latest/syn/struct.Attribute.html#doc-comments
                    if nv.path.is_ident("doc") {
                        match &nv.value {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(doc),
                                ..
                            }) => Some(doc.value()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect()
}
