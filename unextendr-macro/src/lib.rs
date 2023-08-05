use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn unextendr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let item_fn_innner = make_inner_fn(&item_fn);
    let item_fn_outer = make_outer_fn(&item_fn);

    let args_orig = item_fn.sig.inputs.clone();
    let args = item_fn.sig.inputs.iter().map(|i| match i {
        syn::FnArg::Typed(syn::PatType { pat, .. }) => match pat.as_ref() {
            syn::Pat::Ident(arg) => arg.ident.clone(),
            _ => panic!("not supported"),
        },
        _ => panic!("not supported"),
    });

    let item_fn_orig_ts = item_fn.into_token_stream();

    quote! {
        #item_fn_innner
        #item_fn_outer
    }
    .into()
}

fn make_inner_fn(item_fn: &syn::ItemFn) -> syn::ItemFn {
    let mut out = item_fn.clone();

    out.attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));
    out.sig.ident = format_ident!("{}_inner", item_fn.sig.ident);
    out.sig.unsafety = parse_quote!(unsafe);

    out
}

fn make_outer_fn(item_fn: &syn::ItemFn) -> syn::ItemFn {
    let mut out = item_fn.clone();

    let fn_name_outer = item_fn.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", item_fn.sig.ident);
    let args_orig = item_fn.sig.inputs.clone();
    let args = args_orig.iter().map(|i| match i {
        syn::FnArg::Typed(syn::PatType { pat, .. }) => match pat.as_ref() {
            syn::Pat::Ident(arg) => arg.ident.clone(),
            _ => panic!("not supported"),
        },
        _ => panic!("not supported"),
    });

    out.sig.unsafety = parse_quote!(unsafe);

    out.sig.output = parse_quote!(-> SEXP);

    let out: syn::ItemFn = parse_quote!(
        #[allow(clippy::missing_safety_doc)]
        #[no_mangle]
        pub unsafe extern "C" fn #fn_name_outer(#args_orig) -> SEXP {
            unextendr::wrapper(|| #fn_name_inner(#(#args),*))
        }
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn assert_eq_inner(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = make_inner_fn(&orig);
        assert_eq!(result, expected);
    }

    fn assert_eq_outer(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = make_outer_fn(&orig);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_inner_fn() {
        assert_eq_inner(
            parse_quote!(
                #[unextendr]
                fn foo() {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn foo_inner() {
                    bar()
                }
            ),
        );
    }

    #[test]
    fn test_make_outer_fn() {
        assert_eq_outer(
            parse_quote!(
                #[unextendr]
                fn foo() {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn foo() -> SEXP {
                    unextendr::wrapper(|| foo_inner())
                }
            ),
        );
    }
}
