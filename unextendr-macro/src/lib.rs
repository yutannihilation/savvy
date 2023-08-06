use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use unextendr_bindgen::UnextendrFn;

#[proc_macro_attribute]
pub fn unextendr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let unextendr_fn = UnextendrFn::new(&item_fn);

    let item_fn_innner = unextendr_fn.make_inner_fn();
    let item_fn_outer = unextendr_fn.make_outer_fn();

    quote! {
        #item_fn_innner
        #item_fn_outer
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn assert_eq_inner(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = UnextendrFn::new(&orig).make_inner_fn();
        assert_eq!(result, expected);
    }

    fn assert_eq_outer(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = UnextendrFn::new(&orig).make_outer_fn();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_inner_fn() {
        assert_eq_inner(
            parse_quote!(
                #[unextendr]
                fn foo() -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn unextendr_foo_inner() -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            parse_quote!(
                #[unextendr]
                fn foo(x: RealSxp) -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn unextendr_foo_inner(x: unextendr::SEXP) -> unextendr::Result<unextendr::SEXP> {
                    let x = unextendr::RealSxp::try_from(x)?;
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
                fn foo() -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn unextendr_foo() -> unextendr::SEXP {
                    unextendr::wrapper(|| unextendr_foo_inner())
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_outer(
            parse_quote!(
                #[unextendr]
                fn foo(
                    x: RealSxp,
                    y: unextendr::RealSxp,
                ) -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn unextendr_foo(
                    x: unextendr::SEXP,
                    y: unextendr::SEXP
                ) -> unextendr::SEXP {
                    unextendr::wrapper(|| unextendr_foo_inner(x, y))
                }
            ),
        );
    }
}
