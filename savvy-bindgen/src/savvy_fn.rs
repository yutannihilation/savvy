use quote::format_ident;
use syn::{parse_quote, Attribute, Block, FnArg::Typed, Pat::Ident, PatType, Signature, Stmt};

use crate::{savvy_impl::SavvyImpl, utils::extract_docs};

// For main.rs
pub struct ParsedResult {
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<SavvyImpl>,
}

struct SavvyInputType(syn::Type);

#[allow(dead_code)]
impl SavvyInputType {
    fn from_type(ty: &syn::Type) -> syn::Result<Self> {
        if let syn::Type::Path(type_path) = ty {
            let type_ident = &type_path.path.segments.last().unwrap().ident;
            let ty_str = type_ident.to_string();
            match ty_str.as_str() {
                "OwnedIntegerSexp" | "OwnedRealSexp" | "OwnedLogicalSexp" | "OwnedStringSexp"
                | "OwnedListSexp" => {
                    let msg = format!(
                        "`Owned-` types are not allowed here. Did you mean `{}`?",
                        ty_str.strip_prefix("Owned").unwrap()
                    );
                    return Err(syn::Error::new_spanned(type_path, msg));
                }
                _ => {}
            }
        }

        Ok(Self(ty.clone()))
    }

    /// Return the corresponding type for internal function.
    fn to_rust_type_outer(&self) -> syn::Type {
        self.0.clone()
    }

    /// Return the corresponding type for API function (at the moment, only `SEXP` is supported).
    fn to_rust_type_inner(&self) -> syn::Type {
        parse_quote!(savvy::ffi::SEXP)
    }

    /// Return the corresponding type for C function (at the moment, only `SEXP` is supported).
    fn to_c_type(&self) -> String {
        "SEXP".to_string()
    }
}

pub struct SavvyFnArg {
    pat: syn::Ident,
    ty: SavvyInputType,
}

impl SavvyFnArg {
    pub fn pat(&self) -> syn::Ident {
        self.pat.clone()
    }

    pub fn pat_string(&self) -> String {
        self.pat.to_string()
    }

    pub fn to_c_type_string(&self) -> String {
        self.ty.to_c_type()
    }

    pub fn to_rust_type_outer(&self) -> syn::Type {
        self.ty.to_rust_type_outer()
    }

    pub fn to_rust_type_inner(&self) -> syn::Type {
        self.ty.to_rust_type_inner()
    }
}

/// Currently, only `Result::<SEXP>`, `Result<()>`, and Self are supported
pub enum SavvyFnReturnType {
    ResultSexp(syn::ReturnType),
    ResultUnit(syn::ReturnType),
}

impl SavvyFnReturnType {
    pub fn inner(&self) -> &syn::ReturnType {
        match self {
            SavvyFnReturnType::ResultSexp(ret_ty) => ret_ty,
            SavvyFnReturnType::ResultUnit(ret_ty) => ret_ty,
        }
    }
}

pub enum SavvyFnType {
    BareFunction,
    Constructor(syn::Type),
    Method(syn::Type),
    AssociatedFunction(syn::Type),
}

pub struct SavvyFn {
    /// Doc comments
    pub docs: Vec<String>,
    /// Attributes except for `#[savvy]`
    pub attrs: Vec<syn::Attribute>,
    /// Original function name
    pub fn_name: syn::Ident,
    /// type path of `self` in the case of impl function
    pub fn_type: SavvyFnType,
    /// Function arguments
    pub args: Vec<SavvyFnArg>,
    /// Return type of the function
    pub return_type: SavvyFnReturnType,
    /// Original body of the function
    pub stmts_orig: Vec<syn::Stmt>,
    /// Additional lines to convert `SEXP` to the specific types
    pub stmts_additional: Vec<syn::Stmt>,
}

#[allow(dead_code)]
impl SavvyFn {
    pub(crate) fn get_self_ty_ident(&self) -> Option<syn::Ident> {
        let self_ty = match &self.fn_type {
            SavvyFnType::BareFunction => return None,
            SavvyFnType::Constructor(ty) => ty,
            SavvyFnType::Method(ty) => ty,
            SavvyFnType::AssociatedFunction(ty) => ty,
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

    pub fn fn_name_inner(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("savvy_{}_{}_inner", ty, self.fn_name),
            None => format_ident!("savvy_{}_inner", self.fn_name),
        }
    }

    pub fn fn_name_outer(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("{}_{}", ty, self.fn_name),
            None => self.fn_name.clone(),
        }
    }

    pub fn from_fn(orig: &syn::ItemFn) -> syn::Result<Self> {
        Self::new(
            &orig.attrs,
            &orig.sig,
            orig.block.as_ref(),
            SavvyFnType::BareFunction,
        )
    }

    pub fn from_impl_fn(orig: &syn::ImplItemFn, fn_type: SavvyFnType) -> syn::Result<Self> {
        Self::new(&orig.attrs, &orig.sig, &orig.block, fn_type)
    }

    pub fn new(
        attrs: &[Attribute],
        sig: &Signature,
        block: &Block,
        fn_type: SavvyFnType,
    ) -> syn::Result<Self> {
        // TODO: check function signature and abort if any of it is unexpected one.

        let mut attrs = attrs.to_vec();
        // Remove #[savvy]
        attrs.retain(|attr| attr != &parse_quote!(#[savvy]));

        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());

        let fn_name = sig.ident.clone();

        let stmts_orig = block.stmts.clone();
        let mut stmts_additional: Vec<Stmt> = Vec::new();

        let args_new = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                Typed(PatType { pat, ty, .. }) => {
                    let pat = match pat.as_ref() {
                        Ident(arg) => arg.ident.clone(),
                        _ => {
                            return Some(Err(syn::Error::new_spanned(
                                pat,
                                "non-ident is not supported",
                            )));
                        }
                    };

                    let ty = match SavvyInputType::from_type(ty.as_ref()) {
                        Ok(ty) => ty,
                        Err(e) => return Some(Err(e)),
                    };
                    let ty_ident = ty.to_rust_type_outer();

                    stmts_additional.push(parse_quote! {
                        let #pat = <#ty_ident>::try_from(savvy::Sexp(#pat))?;
                    });

                    Some(Ok(SavvyFnArg { pat, ty }))
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
            .collect::<syn::Result<Vec<SavvyFnArg>>>()?;

        Ok(Self {
            docs,
            attrs,
            fn_name,
            fn_type,
            args: args_new,
            return_type: get_savvy_return_type(&sig.output)?,
            stmts_orig,
            stmts_additional,
        })
    }
}

// Allowed return types are the followings. Note that, `Self` is converted to
// `EXTPTRSXP`, so the same as `savvy::Result<savvy::Sexp>`.
//
// - `savvy::Result<savvy::Sexp>`
// - `savvy::Result<()>`
// - `Self`
fn get_savvy_return_type(return_type: &syn::ReturnType) -> syn::Result<SavvyFnReturnType> {
    match return_type {
        syn::ReturnType::Default => Err(syn::Error::new_spanned(
            return_type.clone(),
            "function must have return type",
        )),
        syn::ReturnType::Type(_, ty) => {
            let e = Err(syn::Error::new_spanned(
                return_type.clone(),
                "the return type must be either syn::Result<Sexp> or syn::Result<()>",
            ));

            // Check if the type path is savvy::Result<..> or Result<..> and get
            // the arguments inside < >.
            let path_args = match ty.as_ref() {
                syn::Type::Path(type_path) => {
                    if !is_type_path_savvy_or_no_qualifier(type_path) {
                        return e;
                    }

                    let last_path_seg = type_path.path.segments.last().unwrap();
                    match last_path_seg.ident.to_string().as_str() {
                        "Result" => {}
                        "Self" => {
                            let ret_ty: syn::ReturnType =
                                parse_quote!(-> savvy::Result<savvy::Sexp>);
                            return Ok(SavvyFnReturnType::ResultSexp(ret_ty));
                        }
                        _ => return e,
                    }
                    &last_path_seg.arguments
                }
                _ => return e,
            };

            // Check if the args inside < > is either savvy::Sexp, Sexp, or ()
            match path_args {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) => {
                    if args.len() != 1 {
                        return e;
                    }
                    match &args.first().unwrap() {
                        syn::GenericArgument::Type(ty) => match ty {
                            syn::Type::Tuple(type_tuple) => {
                                if type_tuple.elems.is_empty() {
                                    Ok(SavvyFnReturnType::ResultUnit(return_type.clone()))
                                } else {
                                    e
                                }
                            }
                            syn::Type::Path(type_path) => {
                                if !is_type_path_savvy_or_no_qualifier(type_path) {
                                    return e;
                                }

                                let last_path_seg = type_path.path.segments.last().unwrap();
                                if &last_path_seg.ident.to_string() != "Sexp" {
                                    return e;
                                }

                                Ok(SavvyFnReturnType::ResultSexp(return_type.clone()))
                            }
                            _ => e,
                        },
                        _ => e,
                    }
                }
                _ => e,
            }
        }
    }
}

/// check if the type path either starts with `savvy::` or no qualifier
fn is_type_path_savvy_or_no_qualifier(type_path: &syn::TypePath) -> bool {
    if type_path.qself.is_some() || type_path.path.leading_colon.is_some() {
        return false;
    }

    match type_path.path.segments.len() {
        1 => true,
        2 => {
            let first_path_seg = type_path.path.segments.first().unwrap();
            first_path_seg.arguments.is_none() && &first_path_seg.ident.to_string() == "savvy"
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_detect_return_type() {
        let ok_cases1: &[syn::ReturnType] = &[
            parse_quote!(-> Result<Sexp>),
            parse_quote!(-> savvy::Result<Sexp>),
            parse_quote!(-> savvy::Result<savvy::Sexp>),
            parse_quote!(-> Self),
        ];

        for rt in ok_cases1 {
            let srt = get_savvy_return_type(rt);
            assert!(srt.is_ok());
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::ResultSexp(_)));
        }

        let ok_cases2: &[syn::ReturnType] = &[
            parse_quote!(-> Result<()>),
            parse_quote!(-> savvy::Result<()>),
        ];

        for rt in ok_cases2 {
            let srt = get_savvy_return_type(rt);
            assert!(srt.is_ok());
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::ResultUnit(_)));
        }

        let err_cases: &[syn::ReturnType] = &[
            parse_quote!(-> FOO),
            parse_quote!(-> savvy::Result<FOO>),
            parse_quote!(-> savvy::Result<(T, T)>),
            parse_quote!(-> foo::Result<Sexp>),
            parse_quote!(),
        ];

        for rt in err_cases {
            assert!(get_savvy_return_type(rt).is_err())
        }
    }
}
