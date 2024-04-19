use proc_macro::TokenStream;
use quote::quote;

use savvy_bindgen::{SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};

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

fn savvy_fn(orig: &syn::ItemFn) -> syn::Result<TokenStream> {
    let savvy_fn = SavvyFn::from_fn(orig)?;

    let item_fn_inner = savvy_fn.generate_inner_fn();
    let item_fn_ffi = savvy_fn.generate_ffi_fn();

    Ok(quote! {
        #orig
        #item_fn_inner
        #item_fn_ffi
    }
    .into())
}

fn savvy_impl(item_impl: &syn::ItemImpl) -> syn::Result<TokenStream> {
    let savvy_impl = SavvyImpl::new(item_impl)?;
    let orig = savvy_impl.orig.clone();

    let list_fn_inner = savvy_impl.generate_inner_fns();
    let list_fn_ffi = savvy_impl.generate_ffi_fns();

    Ok(quote! {
        #orig

        #(#list_fn_inner)*
        #(#list_fn_ffi)*
    }
    .into())
}

fn savvy_struct(item_struct: &syn::ItemStruct) -> syn::Result<TokenStream> {
    let savvy_struct = SavvyStruct::new(item_struct)?;
    let orig = &savvy_struct.orig;
    let try_from_impls = savvy_struct.generate_try_from_impls();

    Ok(quote!(
        #orig

        #(#try_from_impls)*
    )
    .into())
}

fn savvy_enum(item_enum: &syn::ItemEnum) -> syn::Result<TokenStream> {
    let savvy_enum = SavvyEnum::new(item_enum)?;
    let enum_with_discriminant = savvy_enum.generate_enum_with_discriminant();
    let try_from_impls = savvy_enum.generate_try_from_impls();

    Ok(quote!(
        #enum_with_discriminant

        #(#try_from_impls)*
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettyplease::unparse;
    use syn::parse_quote;

    fn assert_snapshot_inner(orig: syn::ItemFn) {
        let result = SavvyFn::from_fn(&orig)
            .expect("Failed to parse a function")
            .generate_inner_fn();
        let formatted = unparse(&parse_quote!(#result));
        let lines = formatted.lines().collect::<Vec<&str>>();
        insta::assert_yaml_snapshot!(lines);
    }

    #[test]
    fn test_generate_inner_fn() {
        assert_snapshot_inner(parse_quote!(
            #[savvy]
            fn foo() -> savvy::Result<savvy::Sexp> {
                bar()
            }
        ));

        assert_snapshot_inner(parse_quote!(
            #[savvy]
            fn foo() -> savvy::Result<()> {
                bar();
                Ok(())
            }
        ));

        assert_snapshot_inner(
            // The qualified form (with `savvy::`) and non-qualified form is
            // kept between conversions.
            parse_quote!(
                #[savvy]
                fn foo(x: RealSexp, y: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
                    bar()
                }
            ),
        );

        assert_snapshot_inner(parse_quote!(
            #[savvy]
            fn foo(x: f64) -> savvy::Result<savvy::Sexp> {
                bar()
            }
        ));
    }

    fn assert_snapshot_ffi(orig: syn::ItemFn) {
        let result = SavvyFn::from_fn(&orig)
            .expect("Failed to parse an impl")
            .generate_ffi_fn();
        let formatted = unparse(&parse_quote!(#result));
        let lines = formatted.lines().collect::<Vec<&str>>();
        insta::assert_yaml_snapshot!(lines);
    }

    #[test]
    fn test_generate_ffi_fn() {
        assert_snapshot_ffi(parse_quote!(
            #[savvy]
            fn foo() -> savvy::Result<savvy::Sexp> {
                bar()
            }
        ));

        assert_snapshot_ffi(parse_quote!(
            #[savvy]
            fn foo() -> savvy::Result<()> {
                bar();
                Ok(())
            }
        ));

        assert_snapshot_ffi(parse_quote!(
            #[savvy]
            fn foo(x: RealSexp, y: savvy::RealSexp) -> savvy::Result<savvy::Sexp> {
                bar()
            }
        ));
    }

    fn assert_snapshot_ffi_impl(orig: &syn::ItemImpl) {
        for item_fn in SavvyImpl::new(orig).expect("Failed to parse an impl").fns {
            let result = item_fn.generate_ffi_fn();
            let formatted = unparse(&parse_quote!(#result));
            let lines = formatted.lines().collect::<Vec<&str>>();
            insta::assert_yaml_snapshot!(lines);
        }
    }

    #[test]
    fn test_generate_ffi_fn_impl() {
        assert_snapshot_ffi_impl(&parse_quote!(
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
        ));
    }
}
