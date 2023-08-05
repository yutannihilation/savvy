use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, FnArg::Typed, Pat::Ident, PatType, Stmt};

struct UnextendrFn {
    /// Attributes except for `#[unextendr]`
    attrs: Vec<syn::Attribute>,
    /// Original function name
    fn_name: syn::Ident,
    /// Original arguments (e.g. `x: RealSxp, y: i32`)
    args_orig: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    /// Arguments for API functions (e.g. `x: SEXP, y: i32`)
    args_new: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    /// Arguments to call the inner functions with (e.g. `x, y`)
    args_for_calling: Vec<syn::Ident>,
    /// Original body of the function
    stmts_orig: Vec<syn::Stmt>,
    /// Additional lines to convert `SEXP` to the specific types
    stmts_additional: Vec<syn::Stmt>,
}

impl UnextendrFn {
    fn fn_name_inner(&self) -> syn::Ident {
        format_ident!("unextendr_{}_inner", self.fn_name)
    }

    fn fn_name_outer(&self) -> syn::Ident {
        format_ident!("unextendr_{}", self.fn_name)
    }

    fn new(orig: &syn::ItemFn) -> Self {
        // TODO: check function signature and abort if any of it is unexpected one.

        let mut attrs = orig.attrs.clone();
        // Remove #[unextendr]
        attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));

        let fn_name = orig.sig.ident.clone();

        let args_orig = orig.sig.inputs.clone();
        let mut args_new = args_orig.clone();

        let mut args_for_calling: Vec<syn::Ident> = Vec::new();

        let stmts_orig = orig.block.stmts.clone();
        let mut stmts_additional: Vec<Stmt> = Vec::new();

        for arg in args_new.iter_mut() {
            if let Typed(PatType { pat, ty, .. }) = arg {
                let pat_ident = match pat.as_ref() {
                    Ident(arg) => &arg.ident,
                    _ => panic!("not supported"),
                };

                args_for_calling.push(pat_ident.clone());

                let type_ident = match ty.as_ref() {
                    syn::Type::Path(type_path) => match type_path.path.get_ident() {
                        Some(ident) => ident,
                        None => continue,
                    },
                    _ => continue,
                };

                match type_ident.to_string().as_str() {
                    "IntegerSxp" | "RealSxp" | "LogicalSxp" | "StringSxp" => {
                        stmts_additional.push(parse_quote! {
                            let #pat_ident = unextendr::#type_ident::try_from(#pat_ident)?;
                        });

                        *ty.as_mut() = parse_quote!(unextendr::SEXP);
                    }
                    _ => {}
                }
            }
        }

        Self {
            attrs,
            fn_name,
            args_orig,
            args_new,
            args_for_calling,
            stmts_orig,
            stmts_additional,
        }
    }

    fn make_inner_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();

        let args_new = &self.args_new;
        let stmts_additional = self.stmts_additional.clone();
        let stmts_orig = self.stmts_orig.clone();
        let attrs = self.attrs.clone();

        let out: syn::ItemFn = parse_quote!(
            #(#attrs)*
            unsafe fn #fn_name_inner(#args_new) -> unextendr::Result<unextendr::SEXP> {
                #(#stmts_additional)*
                #(#stmts_orig)*
            }
        );
        out
    }

    fn make_outer_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();
        let fn_name_outer = self.fn_name_outer();

        let args_new = &self.args_new;
        let args_for_calling = &self.args_for_calling;

        let out: syn::ItemFn = parse_quote!(
            #[allow(clippy::missing_safety_doc)]
            #[no_mangle]
            pub unsafe extern "C" fn #fn_name_outer(#args_new) -> unextendr::SEXP {
                unextendr::wrapper(|| #fn_name_inner(#(#args_for_calling),*))
            }
        );
        out
    }
}

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
                unsafe fn unextendr_foo_inner(
                    x: unextendr::SEXP
                ) -> unextendr::Result<unextendr::SEXP> {
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

        assert_eq_outer(
            parse_quote!(
                #[unextendr]
                fn foo(x: RealSxp) -> unextendr::Result<unextendr::SEXP> {
                    bar()
                }
            ),
            parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn unextendr_foo(x: unextendr::SEXP) -> unextendr::SEXP {
                    unextendr::wrapper(|| unextendr_foo_inner(x))
                }
            ),
        );
    }
}
