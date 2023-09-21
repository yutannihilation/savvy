use proc_macro::TokenStream;
use quote::quote;

use savvy_bindgen::{SavvyFn, SavvyImpl};

#[proc_macro_attribute]
pub fn savvy(_args: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(item_fn) = syn::parse::<syn::ItemFn>(input.clone()) {
        return savvy_fn(&item_fn);
    }

    let parse_result = syn::parse::<syn::ItemImpl>(input.clone());
    if let Ok(item_impl) = parse_result {
        return savvy_impl(&item_impl);
    }

    proc_macro::TokenStream::from(
        syn::Error::new(
            parse_result.unwrap_err().span(),
            "#[savvy] macro only accepts `Fn` or `Impl`",
        )
        .into_compile_error(),
    )
}

fn savvy_fn(item_fn: &syn::ItemFn) -> TokenStream {
    let savvy_fn = SavvyFn::from_fn(item_fn);

    let item_fn_inner = savvy_fn.generate_inner_fn();
    let item_fn_outer = savvy_fn.generate_outer_fn();

    quote! {
        #item_fn_inner
        #item_fn_outer
    }
    .into()
}

fn savvy_impl(item_impl: &syn::ItemImpl) -> TokenStream {
    let savvy_impl = SavvyImpl::new(item_impl);
    let orig = savvy_impl.orig.clone();
    let ty = savvy_impl.ty.clone();

    let list_fn_inner = savvy_impl.generate_inner_fns();
    let list_fn_outer = savvy_impl.generate_outer_fns();

    quote! {
        #orig

        impl savvy::IntoExtPtrSxp for #ty {}

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
        let result = SavvyFn::from_fn(&orig).generate_inner_fn();
        assert_eq!(result, expected);
    }

    fn assert_eq_outer(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = SavvyFn::from_fn(&orig).generate_outer_fn();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_inner_fn() {
        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner() -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo(x: RealSxp) -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner(x: savvy::SEXP) -> savvy::Result<savvy::SEXP> {
                    let x = <savvy::RealSxp>::try_from(savvy::Sxp(x))?;
                    bar()
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo(x: f64) -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner(x: savvy::SEXP) -> savvy::Result<savvy::SEXP> {
                    let x = <f64>::try_from(savvy::Sxp(x))?;
                    bar()
                }
            ),
        );
    }

    #[test]
    fn test_generate_outer_fn() {
        assert_eq_outer(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn savvy_foo() -> savvy::SEXP {
                    savvy::handle_result(savvy_foo_inner())
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_outer(
            parse_quote!(
                #[savvy]
                fn foo(
                    x: RealSxp,
                    y: savvy::RealSxp,
                ) -> savvy::Result<savvy::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn savvy_foo(
                    x: savvy::SEXP,
                    y: savvy::SEXP
                ) -> savvy::SEXP {
                    savvy::handle_result(savvy_foo_inner(x, y))
                }
            ),
        );
    }
}
