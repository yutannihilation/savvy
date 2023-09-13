use quote::format_ident;
use syn::{
    parse_quote, Attribute, Block, FnArg::Typed, Item, Pat::Ident, PatType, Signature, Stmt,
};

use crate::utils::extract_docs;

#[allow(clippy::enum_variant_names)]
pub enum UnextendrSupportedTypes {
    IntegerSxp,
    RealSxp,
    LogicalSxp,
    StringSxp,
    ListSxp,
}

#[allow(dead_code)]
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
            "ListSxp" => Some(Self::ListSxp),
            _ => None,
        }
    }

    /// Return the corresponding type for internal function.
    fn to_rust_type_outer(&self) -> syn::Type {
        match &self {
            Self::IntegerSxp => parse_quote!(unextendr::IntegerSxp),
            Self::RealSxp => parse_quote!(unextendr::RealSxp),
            Self::LogicalSxp => parse_quote!(unextendr::LogicalSxp),
            Self::StringSxp => parse_quote!(unextendr::StringSxp),
            Self::ListSxp => parse_quote!(unextendr::ListSxp),
        }
    }

    /// Return the corresponding type for API function (at the moment, only `SEXP` is supported).
    fn to_rust_type_inner(&self) -> syn::Type {
        parse_quote!(unextendr::SEXP)
    }

    /// Return the corresponding type for C function (at the moment, only `SEXP` is supported).
    fn to_c_type(&self) -> String {
        "SEXP".to_string()
    }
}

pub struct UnextendrFnArg {
    pat: syn::Ident,
    ty: UnextendrSupportedTypes,
}

pub enum UnextendrFnType {
    BareFunction,
    Constructor(syn::Type),
    Method(syn::Type),
    AssociatedFunction(syn::Type),
}

pub struct UnextendrFn {
    /// Doc comments
    pub docs: Vec<String>,
    /// Attributes except for `#[unextendr]`
    pub attrs: Vec<syn::Attribute>,
    /// Original function name
    pub fn_name: syn::Ident,
    /// type path of `self` in the case of impl function
    pub fn_type: UnextendrFnType,
    /// Function arguments
    pub args: Vec<UnextendrFnArg>,
    /// Whether the function has return value
    pub has_result: bool,
    /// Original body of the function
    pub stmts_orig: Vec<syn::Stmt>,
    /// Additional lines to convert `SEXP` to the specific types
    pub stmts_additional: Vec<syn::Stmt>,
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
        Some(UnextendrFn::from_fn(func))
    } else {
        None
    }
}

#[allow(dead_code)]
impl UnextendrFn {
    fn get_self_ty_ident(&self) -> Option<syn::Ident> {
        let self_ty = match &self.fn_type {
            UnextendrFnType::BareFunction => return None,
            UnextendrFnType::Constructor(ty) => ty,
            UnextendrFnType::Method(ty) => ty,
            UnextendrFnType::AssociatedFunction(ty) => ty,
        };
        if let syn::Type::Path(type_path) = self_ty {
            let ty = type_path
                .path
                .segments
                .last()
                .expect("Unexpected type path")
                .ident
                .clone();
            Some(ty)
        } else {
            panic!("Unexpected self type!")
        }
    }

    pub fn fn_name_orig(&self) -> syn::Ident {
        self.fn_name.clone()
    }

    pub fn fn_name_inner(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("unextendr_{}_{}_inner", ty, self.fn_name),
            None => format_ident!("unextendr_{}_inner", self.fn_name),
        }
    }

    pub fn fn_name_outer(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("unextendr_{}_{}", ty, self.fn_name),
            None => format_ident!("unextendr_{}", self.fn_name),
        }
    }

    pub fn from_fn(orig: &syn::ItemFn) -> Self {
        Self::new(
            &orig.attrs,
            &orig.sig,
            orig.block.as_ref(),
            UnextendrFnType::BareFunction,
        )
    }

    pub fn from_impl_fn(orig: &syn::ImplItemFn, fn_type: UnextendrFnType) -> Self {
        Self::new(&orig.attrs, &orig.sig, &orig.block, fn_type)
    }

    pub fn new(
        attrs: &[Attribute],
        sig: &Signature,
        block: &Block,
        fn_type: UnextendrFnType,
    ) -> Self {
        // TODO: check function signature and abort if any of it is unexpected one.

        let mut attrs = attrs.to_vec();
        // Remove #[unextendr]
        attrs.retain(|attr| attr != &parse_quote!(#[unextendr]));

        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());

        let fn_name = sig.ident.clone();

        let stmts_orig = block.stmts.clone();
        let mut stmts_additional: Vec<Stmt> = Vec::new();

        let args_new: Vec<UnextendrFnArg> = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                Typed(PatType { pat, ty, .. }) => {
                    let pat = match pat.as_ref() {
                        Ident(arg) => arg.ident.clone(),
                        _ => panic!("non-ident is not supported"),
                    };

                    let ty = UnextendrSupportedTypes::from_type(ty.as_ref())
                        .expect("the type is not supported");

                    let ty_ident = ty.to_rust_type_outer();

                    stmts_additional.push(parse_quote! {
                        let #pat = #ty_ident::try_from(#pat)?;
                    });

                    Some(UnextendrFnArg { pat, ty })
                }
                // Skip `self`
                syn::FnArg::Receiver(syn::Receiver { reference, .. }) => {
                    if reference.is_none() {
                        // TODO: raise compile error if no reference.
                        // The function should not consume the object
                        // because the EXTPTRSXP still live even after
                        // the function returns.
                    }

                    None
                }
            })
            .collect();

        let has_result = match sig.output {
            syn::ReturnType::Default => false,
            syn::ReturnType::Type(_, _) => true,
        };

        Self {
            docs,
            attrs,
            fn_name,
            fn_type,
            args: args_new,
            has_result,
            stmts_orig,
            stmts_additional,
        }
    }

    pub fn make_inner_fn(&self) -> syn::ItemFn {
        let fn_name_orig = &self.fn_name;
        let fn_name_inner = self.fn_name_inner();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat.clone()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.ty.to_rust_type_inner())
            .collect();

        let stmts_additional = &self.stmts_additional;
        let stmts_orig = &self.stmts_orig;
        let attrs = &self.attrs;

        let out: syn::ItemFn = match (&self.fn_type, self.has_result) {
            // A bare function with result
            (UnextendrFnType::BareFunction, true) => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    #(#stmts_additional)*
                    #(#stmts_orig)*
                }
            ),
            // A bare function without result; return a dummy value
            (UnextendrFnType::BareFunction, false) => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    #(#stmts_additional)*
                    #(#stmts_orig)*

                    // Dummy return value
                    Ok(unextendr::NullSxp.into())
                }
            ),
            // A method with result
            (UnextendrFnType::Method(ty), true) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(self__: unextendr::SEXP, #(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    let self__ = unextendr::get_external_pointer_addr(self__) as *mut #ty;
                    #(#stmts_additional)*

                    (*self__).#fn_name_orig(#(#args_pat),*)
                }
            ),
            // A method without result; return a dummy value
            (UnextendrFnType::Method(ty), false) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(self__: unextendr::SEXP, #(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    let self__ = unextendr::get_external_pointer_addr(self__) as *mut #ty;
                    #(#stmts_additional)*

                    (*self__).#fn_name_orig(#(#args_pat),*);

                    // Dummy return value
                    Ok(unextendr::NullSxp.into())
                }
            ),
            // An associated function with a result
            (UnextendrFnType::AssociatedFunction(ty), true) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    #(#stmts_additional)*

                    #ty::#fn_name_orig(#(#args_pat),*)
                }
            ),
            // An associated function without result; return a dummy value
            (UnextendrFnType::AssociatedFunction(ty), false) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    #(#stmts_additional)*

                    #ty::#fn_name_orig(#(#args_pat),*);

                    // Dummy return value
                    Ok(unextendr::NullSxp.into())
                }
            ),
            (UnextendrFnType::Constructor(ty), _) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> unextendr::Result<unextendr::SEXP> {
                    #(#stmts_additional)*
                    let x = #ty::#fn_name_orig(#(#args_pat),*);
                    Ok(x.into_external_pointer())
                }
            ),
        };
        out
    }

    pub fn make_outer_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();
        let fn_name_outer = self.fn_name_outer();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat.clone()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.ty.to_rust_type_inner())
            .collect();

        let out: syn::ItemFn = match &self.fn_type {
            // if the function is a method, add `self__` to the first argument
            UnextendrFnType::Method(_) => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer(self__: unextendr::SEXP, #(#args_pat: #args_ty),* ) -> unextendr::SEXP {
                    unextendr::handle_result(#fn_name_inner(self__, #(#args_pat),*))
                }
            ),
            _ => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer( #(#args_pat: #args_ty),* ) -> unextendr::SEXP {
                    unextendr::handle_result(#fn_name_inner(#(#args_pat),*))
                }
            ),
        };
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

        let body = if self.has_result {
            format!(".Call({fn_name_c}, {args})")
        } else {
            // If the result is NULL, wrap it with invisible
            format!("invisible(.Call({fn_name_c}, {args}))")
        };

        format!(
            "{doc_comments}
{fn_name} <- function({args}) {{
  {body}
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
