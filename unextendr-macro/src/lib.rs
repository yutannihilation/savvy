use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, FnArg::Typed, Pat::Ident, PatType, Stmt};

#[proc_macro_attribute]
pub fn unextendr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let item_fn_innner = make_inner_fn(&item_fn);
    let item_fn_outer = make_outer_fn(&item_fn);

    quote! {
        #item_fn_innner
        #item_fn_outer
    }
    .into()
}

fn make_inner_fn(item_fn: &syn::ItemFn) -> syn::ItemFn {
    let mut out = item_fn.clone();

    // Remove #[unextendr]
    out.attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));

    out.sig.ident = format_ident!("{}_inner", item_fn.sig.ident);
    out.sig.unsafety = parse_quote!(unsafe);

    let mut new_stmts: Vec<Stmt> = Vec::new();
    for arg in out.sig.inputs.iter_mut() {
        if let Typed(PatType { pat, ty, .. }) = arg {
            let pat_ident = match pat.as_ref() {
                Ident(arg) => &arg.ident,
                _ => panic!("not supported"),
            };
            let type_ident = match ty.as_ref() {
                syn::Type::Path(type_path) => match type_path.path.get_ident() {
                    Some(ident) => ident,
                    None => continue,
                },
                _ => continue,
            };

            match type_ident.to_string().as_str() {
                "IntegerSxp" | "RealSxp" | "LogicalSxp" | "StringSxp" => {
                    new_stmts.push(parse_quote! {
                        let #pat_ident = unextendr::#type_ident::try_from(#pat_ident)?;
                    });

                    *ty.as_mut() = parse_quote!(unextendr::SEXP);
                }
                _ => {}
            }
        }
    }

    // Prepend the statements of conversion from SEXP
    new_stmts.append(&mut out.block.stmts);
    out.block.stmts = new_stmts;

    out
}

fn make_outer_fn(item_fn: &syn::ItemFn) -> syn::ItemFn {
    let mut out = item_fn.clone();

    // function names
    let fn_name_outer = out.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", out.sig.ident);

    out.sig.unsafety = parse_quote!(unsafe);
    out.sig.output = parse_quote!(-> SEXP);

    // arguments
    let mut args = out.sig.inputs.clone();
    let mut args_for_calling: Vec<syn::Ident> = Vec::new();
    for arg in args.iter_mut() {
        if let Typed(PatType { pat, ty, .. }) = arg {
            let pat_ident = match pat.as_ref() {
                Ident(arg) => arg.ident.clone(),
                _ => panic!("not supported"),
            };
            let type_ident = match ty.as_ref() {
                syn::Type::Path(type_path) => match type_path.path.get_ident() {
                    Some(ident) => ident,
                    None => continue,
                },
                _ => continue,
            };

            args_for_calling.push(pat_ident);

            match type_ident.to_string().as_str() {
                "IntegerSxp" | "RealSxp" | "LogicalSxp" | "StringSxp" => {
                    *ty.as_mut() = parse_quote!(unextendr::SEXP);
                }
                _ => {}
            }
        }
    }

    let out: syn::ItemFn = parse_quote!(
        #[allow(clippy::missing_safety_doc)]
        #[no_mangle]
        pub unsafe extern "C" fn #fn_name_outer(#args) -> SEXP {
            unextendr::wrapper(|| #fn_name_inner(#(#args_for_calling),*))
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

        assert_eq_inner(
            parse_quote!(
                #[unextendr]
                fn foo(x: RealSxp) {
                    bar()
                }
            ),
            parse_quote!(
                unsafe fn foo_inner(x: unextendr::SEXP) {
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

        assert_eq_outer(
            parse_quote!(
                #[unextendr]
                fn foo(x: RealSxp) {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn foo(x: unextendr::SEXP) -> SEXP {
                    unextendr::wrapper(|| foo_inner(x))
                }
            ),
        );
    }
}
