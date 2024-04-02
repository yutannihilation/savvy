use proc_macro::TokenStream;
use quote::quote;

use savvy_bindgen::{SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};
use syn::parse_quote;

#[proc_macro_attribute]
pub fn savvy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let result = if let Ok(item_fn) = syn::parse::<syn::ItemFn>(input.clone()) {
        savvy_fn(&item_fn)
    } else if let Ok(item_struct) = syn::parse::<syn::ItemStruct>(input.clone()) {
        savvy_struct(&item_struct)
    } else if let Ok(item_impl) = syn::parse::<syn::ItemImpl>(input.clone()) {
        savvy_impl(&item_impl)
    } else if let Ok(item_enum) = syn::parse::<syn::ItemEnum>(input.clone()) {
        savvy_enum(&item_enum)
    } else {
        let parse_result = syn::parse::<syn::ItemImpl>(input.clone());
        return proc_macro::TokenStream::from(
            syn::Error::new(
                parse_result.unwrap_err().span(),
                "#[savvy] macro only accepts `fn`, `struct`, or `impl`",
            )
            .into_compile_error(),
        );
    };

    match result {
        Ok(token_stream) => token_stream,
        Err(e) => e.into_compile_error().into(),
    }
}

fn savvy_fn(item_fn: &syn::ItemFn) -> syn::Result<TokenStream> {
    let savvy_fn = SavvyFn::from_fn(item_fn)?;

    let item_fn_inner = savvy_fn.generate_inner_fn();
    let item_fn_outer = savvy_fn.generate_outer_fn();

    Ok(quote! {
        #item_fn_inner
        #item_fn_outer
    }
    .into())
}

fn savvy_impl(item_impl: &syn::ItemImpl) -> syn::Result<TokenStream> {
    let savvy_impl = SavvyImpl::new(item_impl)?;
    let orig = savvy_impl.orig.clone();

    let list_fn_inner = savvy_impl.generate_inner_fns();
    let list_fn_outer = savvy_impl.generate_outer_fns();

    Ok(quote! {
        #orig

        #(#list_fn_inner)*
        #(#list_fn_outer)*
    }
    .into())
}

fn savvy_struct(item_struct: &syn::ItemStruct) -> syn::Result<TokenStream> {
    let savvy_struct = SavvyStruct::new(item_struct)?;
    let orig = savvy_struct.orig.clone();
    let try_from_impls = savvy_struct.generate_try_from_impls();

    Ok(quote!(
        #orig

        #(#try_from_impls)*
    )
    .into())
}

fn savvy_enum(item_enum: &syn::ItemEnum) -> syn::Result<TokenStream> {
    let SavvyEnum {
        attrs,
        ty,
        variants,
        ..
    } = SavvyEnum::new(item_enum)?;

    let variants_tweaked = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let lit_i = syn::LitInt::new(&i.to_string(), v.span());
            parse_quote!(#v = #lit_i)
        })
        .collect::<Vec<syn::Variant>>();

    let match_arms = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let lit_i = syn::LitInt::new(&i.to_string(), v.span());
            parse_quote!(#lit_i => Ok(#ty::#v))
        })
        .collect::<Vec<syn::Arm>>();

    Ok(quote!(
        #(#attrs)*
        pub enum #ty {
            #(#variants_tweaked),*
        }

        impl TryFrom<#ty> for savvy::Sexp {
            type Error = savvy::Error;

            fn try_from(value: #ty) -> savvy::Result<Self> {
                (value as i32).try_into()
            }
        }

        impl TryFrom<savvy::Sexp> for #ty {
            type Error = savvy::Error;

            fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                let i = <i32>::try_from(value)?;
                match i {
                    #(#match_arms),*,
                    _ => Err("Unexpected enum variant".into()),
                }
            }
        }
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn assert_eq_inner(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = SavvyFn::from_fn(&orig)
            .expect("Failed to parse a function")
            .generate_inner_fn();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_inner_fn() {
        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner() -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
        );

        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<()> {
                    bar();
                    Ok(())
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner() -> savvy::Result<()> {
                    bar();
                    Ok(())
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            // The qualified form (with `savvy::`) and non-qualified form is
            // kept between conversions.
            parse_quote!(
                #[savvy]
                fn foo(x: RealSexp, y: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner(x: savvy::ffi::SEXP, y: savvy::ffi::SEXP) -> savvy::Result<savvy::Sexp> {
                    let x = <RealSexp>::try_from(savvy::Sexp(x))?;
                    let y = <savvy::IntegerSexp>::try_from(savvy::Sexp(y))?;
                    bar()
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_inner(
            parse_quote!(
                #[savvy]
                fn foo(x: f64) -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn savvy_foo_inner(x: savvy::ffi::SEXP) -> savvy::Result<savvy::Sexp> {
                    let x = <f64>::try_from(savvy::Sexp(x))?;
                    bar()
                }
            ),
        );
    }

    fn assert_eq_outer(orig: syn::ItemFn, expected: syn::ItemFn) {
        let result = SavvyFn::from_fn(&orig)
            .expect("Failed to parse an impl")
            .generate_outer_fn();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_outer_fn() {
        assert_eq_outer(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn foo() -> savvy::ffi::SEXP {
                    match savvy_foo_inner() {
                        Ok(result) => result.0,
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
        );

        assert_eq_outer(
            parse_quote!(
                #[savvy]
                fn foo() -> savvy::Result<()> {
                    bar();
                    Ok(())
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn foo() -> savvy::ffi::SEXP {
                    match savvy_foo_inner() {
                        Ok(_) => savvy::sexp::null::null(),
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
        );

        #[rustfmt::skip]
        assert_eq_outer(
            parse_quote!(
                #[savvy]
                fn foo(
                    x: RealSexp,
                    y: savvy::RealSexp,
                ) -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn foo(
                    x: savvy::ffi::SEXP,
                    y: savvy::ffi::SEXP
                ) -> savvy::ffi::SEXP {
                    match savvy_foo_inner(x, y) {
                        Ok(result) => result.0,
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
        );
    }

    fn assert_eq_outer_impl(orig: &syn::ItemImpl, expected: syn::ItemFn, i: usize) {
        let result =
            SavvyImpl::new(orig).expect("Failed to parse an impl").fns[i].generate_outer_fn();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_outer_fn_impl() {
        let impl1: syn::ItemImpl = parse_quote!(
            #[savvy]
            impl Person {
                fn new() -> Self {
                    Self {}
                }
                fn new2() -> Person {
                    Person {}
                }
                fn name(&self) -> savvy::Result<savvy::Sexp> {
                    Ok(out.into())
                }
                fn set_name(&self, name: StringSexp) -> Result<()> {
                    Ok(())
                }
            }
        );

        #[rustfmt::skip]
        assert_eq_outer_impl(
            &impl1,
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn Person_new() -> savvy::ffi::SEXP {
                    match savvy_Person_new_inner() {
                        Ok(result) => {
                            use savvy::IntoExtPtrSexp;
                            result.into_external_pointer().0
                        },
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
            0,
        );

        #[rustfmt::skip]
        assert_eq_outer_impl(
            &impl1,
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn Person_new2() -> savvy::ffi::SEXP {
                    match savvy_Person_new2_inner() {
                        Ok(result) => {
                            use savvy::IntoExtPtrSexp;
                            result.into_external_pointer().0
                        },
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
            1,
        );

        #[rustfmt::skip]
        assert_eq_outer_impl(
            &impl1,
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn Person_name(
                    self__: savvy::ffi::SEXP,
                ) -> savvy::ffi::SEXP {
                    match savvy_Person_name_inner(self__, ) {
                        Ok(result) => result.0,
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
            2,
        );

        #[rustfmt::skip]
        assert_eq_outer_impl(
            &impl1,
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn Person_set_name(
                    self__: savvy::ffi::SEXP,
                    name: savvy::ffi::SEXP
                ) -> savvy::ffi::SEXP {
                    match savvy_Person_set_name_inner(self__, name) {
                        Ok(_) => savvy::sexp::null::null(),
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
            3,
        );
    }
}
