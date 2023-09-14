use proc_macro::TokenStream;
use quote::quote;

use unextendr_bindgen::{UnextendrFn, UnextendrImpl};

#[proc_macro_attribute]
pub fn unextendr(_args: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(item_fn) = syn::parse::<syn::ItemFn>(input.clone()) {
        return unextendr_fn(&item_fn);
    }

    let parse_result = syn::parse::<syn::ItemImpl>(input.clone());
    if let Ok(item_impl) = parse_result {
        return unextendr_impl(&item_impl);
    }

    proc_macro::TokenStream::from(
        syn::Error::new(
            parse_result.unwrap_err().span(),
            "#[unextendr] macro only accepts `Fn` or `Impl`",
        )
        .into_compile_error(),
    )
}

fn unextendr_fn(item_fn: &syn::ItemFn) -> TokenStream {
    let unextendr_fn = UnextendrFn::from_fn(item_fn);

    let item_fn_inner = unextendr_fn.make_inner_fn();
    let item_fn_outer = unextendr_fn.make_outer_fn();

    quote! {
        #item_fn_inner
        #item_fn_outer
    }
    .into()
}

fn unextendr_impl(item_impl: &syn::ItemImpl) -> TokenStream {
    let unextendr_impl = UnextendrImpl::new(item_impl);
    let orig = unextendr_impl.orig.clone();
    let ty = unextendr_impl.ty.clone();

    let list_fn_inner = unextendr_impl.make_inner_fns();
    let list_fn_outer = unextendr_impl.make_outer_fns();

    quote! {
        #orig

        impl unextendr::IntoExtPtrSxp for #ty {}

        #(#list_fn_inner)*
        #(#list_fn_outer)*
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn assert_eq_inner(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = UnextendrFn::from_fn(&orig).make_inner_fn();
        assert_eq!(result, expected);
    }

    fn assert_eq_outer(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = UnextendrFn::from_fn(&orig).make_outer_fn();
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
                    let x = <unextendr::RealSxp>::try_from(unextendr::Sxp(x))?;
                    bar()
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            parse_quote!(
                #[unextendr]
                fn foo(x: f64) -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn unextendr_foo_inner(x: unextendr::SEXP) -> unextendr::Result<unextendr::SEXP> {
                    let x = <f64>::try_from(unextendr::Sxp(x))?;
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
                    unextendr::handle_result(unextendr_foo_inner())
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
                    unextendr::handle_result(unextendr_foo_inner(x, y))
                }
            ),
        );
    }
}
