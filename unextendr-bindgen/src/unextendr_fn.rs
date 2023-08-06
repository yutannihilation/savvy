use quote::format_ident;
use syn::{parse_quote, FnArg::Typed, Item, Pat::Ident, PatType, Stmt};

pub enum UnextendrSupportedTypes {
    IntegerSxp,
    RealSxp,
    LogicalSxp,
    StringSxp,
}

impl UnextendrSupportedTypes {
    fn from_type(ty: &syn::Type) -> Option<Self> {
        // Use only the last part to support both the qualified type path (e.g.,
        // `unextendr::IntegerSxp`), and single type (e.g., `IntegerSxp`)
        let type_ident = match ty {
            syn::Type::Path(type_path) => &type_path.path.segments.last().unwrap().ident,
            _ => {
                return None;
            }
        };

        match type_ident.to_string().as_str() {
            "IntegerSxp" => Some(Self::IntegerSxp),
            "RealSxp" => Some(Self::RealSxp),
            "LogicalSxp" => Some(Self::LogicalSxp),
            "StringSxp" => Some(Self::StringSxp),
            _ => None,
        }
    }

    /// Return the corresponding type for internal function.
    fn to_rust_type_inner(&self) -> syn::Type {
        match &self {
            Self::IntegerSxp => parse_quote!(unextendr::IntegerSxp),
            Self::RealSxp => parse_quote!(unextendr::RealSxp),
            Self::LogicalSxp => parse_quote!(unextendr::LogicalSxp),
            Self::StringSxp => parse_quote!(unextendr::StringSxp),
        }
    }

    /// Return the corresponding type for API function (at the moment, only `SEXP` is supported).
    fn to_rust_type_outer(&self) -> syn::Type {
        match &self {
            Self::IntegerSxp | Self::RealSxp | Self::LogicalSxp | Self::StringSxp => {
                parse_quote!(unextendr::SEXP)
            }
        }
    }

    /// Return the corresponding type for C function (at the moment, only `SEXP` is supported).
    fn to_c_type(&self) -> String {
        match &self {
            Self::IntegerSxp | Self::RealSxp | Self::LogicalSxp | Self::StringSxp => "SEXP",
        }
        .to_string()
    }
}

pub struct UnextendrFnArg {
    pat: syn::Ident,
    ty: UnextendrSupportedTypes,
}

pub struct UnextendrFn {
    /// Doc comments
    docs: Vec<String>,
    /// Attributes except for `#[unextendr]`
    attrs: Vec<syn::Attribute>,
    /// Original function name
    fn_name: syn::Ident,
    /// Function arguments
    args: Vec<UnextendrFnArg>,
    /// Original body of the function
    stmts_orig: Vec<syn::Stmt>,
    /// Additional lines to convert `SEXP` to the specific types
    stmts_additional: Vec<syn::Stmt>,
}

pub fn parse_unextendr_fn(item: &Item) -> Option<UnextendrFn> {
    let func = match item {
        syn::Item::Fn(func) => func,
        _ => {
            return None;
        }
    };

    // Generate bindings only when the function is marked by #[unextendr]
    if func
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[unextendr]))
    {
        Some(UnextendrFn::new(func))
    } else {
        None
    }
}

impl UnextendrFn {
    pub fn fn_name_orig(&self) -> syn::Ident {
        self.fn_name.clone()
    }

    pub fn fn_name_inner(&self) -> syn::Ident {
        format_ident!("unextendr_{}_inner", self.fn_name)
    }

    pub fn fn_name_outer(&self) -> syn::Ident {
        format_ident!("unextendr_{}", self.fn_name)
    }

    pub fn new(orig: &syn::ItemFn) -> Self {
        // TODO: check function signature and abort if any of it is unexpected one.

        let mut attrs = orig.attrs.clone();
        // Remove #[unextendr]
        attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));

        // Extract doc comments
        let docs = attrs
            .iter()
            .filter_map(|attr| {
                match &attr.meta {
                    syn::Meta::NameValue(nv) => {
                        // Doc omments are transformed into the form of `#[doc =
                        // r"comment"]` before macros are expanded.
                        // cf., https://docs.rs/syn/latest/syn/struct.Attribute.html#doc-comments
                        if nv.path.is_ident("doc") {
                            match &nv.value {
                                syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(doc),
                                    ..
                                }) => Some(doc.value()),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        let fn_name = orig.sig.ident.clone();

        let stmts_orig = orig.block.stmts.clone();
        let mut stmts_additional: Vec<Stmt> = Vec::new();

        let args_new: Vec<UnextendrFnArg> = orig
            .sig
            .inputs
            .iter()
            .map(|arg| {
                if let Typed(PatType { pat, ty, .. }) = arg {
                    let pat = match pat.as_ref() {
                        Ident(arg) => arg.ident.clone(),
                        _ => panic!("not supported"),
                    };

                    let ty =
                        UnextendrSupportedTypes::from_type(ty.as_ref()).expect("not supported");

                    let ty_ident = ty.to_rust_type_inner();

                    stmts_additional.push(parse_quote! {
                        let #pat = #ty_ident::try_from(#pat)?;
                    });

                    UnextendrFnArg { pat, ty }
                } else {
                    panic!("not supported");
                }
            })
            .collect();

        Self {
            docs,
            attrs,
            fn_name,
            args: args_new,
            stmts_orig,
            stmts_additional,
        }
    }

    pub fn make_inner_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat.clone()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.ty.to_rust_type_outer())
            .collect();

        let stmts_additional = &self.stmts_additional;
        let stmts_orig = &self.stmts_orig;
        let attrs = &self.attrs;

        let out: syn::ItemFn = parse_quote!(
            #(#attrs)*
            unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                #(#stmts_additional)*
                #(#stmts_orig)*
            }
        );
        out
    }

    pub fn make_outer_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();
        let fn_name_outer = self.fn_name_outer();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat.clone()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.ty.to_rust_type_outer())
            .collect();

        let out: syn::ItemFn = parse_quote!(
            #[allow(clippy::missing_safety_doc)]
            #[no_mangle]
            pub unsafe extern "C" fn #fn_name_outer( #(#args_pat: #args_ty),* ) -> unextendr::SEXP {
                unextendr::handle_result(#fn_name_inner(#(#args_pat),*))
            }
        );
        out
    }

    /// Generate C function signature
    fn to_c_function_for_header(&self) -> String {
        let fn_name = self.fn_name_outer();
        let args = self
            .args
            .iter()
            .map(|arg| {
                let pat = &arg.pat;
                let ty = arg.ty.to_c_type();
                format!("{ty} {pat}")
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("SEXP {fn_name}({args});")
    }

    /// Generate C function implementation
    fn to_c_function_impl(&self) -> String {
        let fn_name = self.fn_name_outer();
        format!(
            "
SEXP {fn_name}_wrapper(SEXP x) {{
    SEXP res = {fn_name}(x);
    return handle_result(res);
}}"
        )
    }

    /// Generate C function call entry
    fn to_c_function_call_entry(&self) -> String {
        let fn_name = self.fn_name_outer();
        let n_args = self.args.len();
        format!(r#"    {{"{fn_name}", (DL_FUNC) &{fn_name}_wrapper, {n_args}}},"#)
    }

    fn to_r_function(&self) -> String {
        let fn_name = self.fn_name_orig();
        let fn_name_c = self.fn_name_outer();

        let doc_comments = self
            .docs
            .iter()
            .map(|doc| format!("#'{doc}"))
            .collect::<Vec<String>>()
            .join("\n");

        let args = self
            .args
            .iter()
            .map(|arg| arg.pat.clone().to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "{doc_comments}
{fn_name} <- function({args}) {{
  .Call({fn_name_c}, {args})
}}
"
        )
    }
}

pub fn make_c_header_file(fns: &[UnextendrFn]) -> String {
    fns.iter()
        .map(|x| x.to_c_function_for_header())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn make_c_impl_file(fns: &[UnextendrFn]) -> String {
    let common_part = r#"
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_error() directly.
            Rf_error("%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}
"#;

    let c_fns = fns
        .iter()
        .map(|x| x.to_c_function_impl())
        .collect::<Vec<String>>()
        .join("\n");

    let call_entries = fns
        .iter()
        .map(|x| x.to_c_function_call_entry())
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{common_part}
{c_fns}

static const R_CallMethodDef CallEntries[] = {{
{call_entries}
    {{NULL, NULL, 0}}
}};

void R_init_unextendr(DllInfo *dll) {{
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}}
"
    )
}

pub fn make_r_impl_file(fns: &[UnextendrFn]) -> String {
    let r_fns = fns
        .iter()
        .map(|x| x.to_r_function())
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#' @useDynLib unextendr, .registration = TRUE
#' @keywords internal
"_PACKAGE"

{r_fns}"#
    )
}
