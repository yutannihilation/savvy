use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

struct UnextendrFn {
    name: syn::Ident,
}

#[proc_macro_attribute]
pub fn unextendr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(input as syn::ItemFn);

    let mut item_fn_orig = item_fn.clone();

    make_inner_fn(&mut item_fn);

    item_fn.into_token_stream().into()
}

fn make_inner_fn(item_fn: &mut syn::ItemFn) {
    item_fn.sig.ident = proc_macro2::Ident::new(
        &format!("{}_inner", item_fn.sig.ident),
        proc_macro2::Span::call_site(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_make_inner_fn() {
        let mut item_fn: syn::ItemFn = parse_quote!(
            #[unextendr]
            fn foo() {
                bar()
            }
        );

        make_inner_fn(&mut item_fn);

        assert_eq!(item_fn.sig.ident.to_string(), "foo_inner".to_string())
    }
}
