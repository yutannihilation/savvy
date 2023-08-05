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
    out.sig.ident = format_ident!("{}_inner", item_fn.sig.ident);
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

    #[test]
    fn test_make_inner_fn() {
        let item_fn: syn::ItemFn = parse_quote!(
            #[unextendr]
            fn foo() {
                bar()
            }
        );

        let item_fn_inner = make_inner_fn(&item_fn);

        assert_eq!(item_fn_inner.sig.ident.to_string(), "foo_inner".to_string());
    }

    #[test]
    fn test_make_outer_fn() {
        let item_fn: syn::ItemFn = parse_quote!(
            #[unextendr]
            fn foo() {
                bar()
            }
        );

        let item_fn_outer = make_outer_fn(&item_fn);

        // check function name
        assert_eq!(item_fn_outer.sig.ident.to_string(), "foo".to_string());

        // check attributes
        let expected_attr: Vec<syn::Attribute> = vec![
            parse_quote!(#[allow(clippy::missing_safety_doc)]),
            parse_quote!(#[no_mangle]),
        ];
        assert_eq!(item_fn_outer.attrs, expected_attr);
    }
}
