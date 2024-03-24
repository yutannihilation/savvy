use quote::format_ident;
use syn::{parse_quote, Attribute, Block, FnArg::Typed, Pat::Ident, PatType, Signature, Stmt};

use crate::{savvy_impl::SavvyImpl, utils::extract_docs};

// For main.rs
pub struct ParsedResult {
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<SavvyImpl>,
}

enum SavvyInputTypeCategory {
    SexpWrapper,
    PrimitiveType,
    UserDefinedType,
}

struct SavvyInputType {
    category: SavvyInputTypeCategory,
    ty_orig: syn::Type,
    ty_str: String,
}

#[allow(dead_code)]
impl SavvyInputType {
    fn from_type(ty: &syn::Type) -> syn::Result<Self> {
        match &ty {
            // User-defined structs are accepted in the form of either `&` or
            // `&mut`. Note that `&str` also falls here.
            syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                if let syn::Type::Path(type_path) = elem.as_ref() {
                    let ty_str = type_path.path.segments.last().unwrap().ident.to_string();
                    if &ty_str == "str" {
                        Ok(Self {
                            category: SavvyInputTypeCategory::PrimitiveType,
                            ty_orig: ty.clone(),
                            ty_str: "&str".to_string(),
                        })
                    } else {
                        Ok(Self {
                            category: SavvyInputTypeCategory::UserDefinedType,
                            ty_orig: ty.clone(),
                            ty_str,
                        })
                    }
                } else {
                    Err(syn::Error::new_spanned(
                        ty.clone(),
                        "Unexpected type specification: {:?}",
                    ))
                }
            }

            syn::Type::Path(type_path) => {
                let type_ident = &type_path.path.segments.last().unwrap().ident;
                let ty_str = type_ident.to_string();
                match ty_str.as_str() {
                    // Owned-types are not allowed for the input
                    "OwnedIntegerSexp" | "OwnedRealSexp" | "OwnedLogicalSexp"
                    | "OwnedStringSexp" | "OwnedListSexp" => {
                        let msg = format!(
                            "`Owned-` types are not allowed here. Did you mean `{}`?",
                            ty_str.strip_prefix("Owned").unwrap()
                        );
                        Err(syn::Error::new_spanned(type_path, msg))
                    }

                    // Read-only types
                    "Sexp" | "IntegerSexp" | "RealSexp" | "LogicalSexp" | "StringSexp"
                    | "ListSexp" | "FunctionSexp" => Ok(Self {
                        category: SavvyInputTypeCategory::SexpWrapper,
                        ty_orig: ty.clone(),
                    ty_str
                }),

                    // Primitive types
                    "i32" | "usize" | "f64" | "bool" => Ok(Self {
                        category: SavvyInputTypeCategory::PrimitiveType,
                        ty_orig: ty.clone(),
                    ty_str
                }),

                    _ => Err(syn::Error::new_spanned(
                        type_path,
                        format!("A user-defined struct must be in the form of either `&{ty_str}` or `&mut {ty_str}`"),
                    )),
                }
            }
            _ => Err(syn::Error::new_spanned(
                ty.clone(),
                "Unexpected type specification: {:?}",
            )),
        }
    }

    /// Return the corresponding type for internal function.
    fn to_rust_type_outer(&self) -> syn::Type {
        match &self.category {
            SavvyInputTypeCategory::SexpWrapper => self.ty_orig.clone(),
            SavvyInputTypeCategory::PrimitiveType => self.ty_orig.clone(),
            SavvyInputTypeCategory::UserDefinedType => self.ty_orig.clone(),
        }
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

    pub fn is_user_defined_type(&self) -> bool {
        matches!(&self.ty.category, SavvyInputTypeCategory::UserDefinedType)
    }

    pub fn pat_string(&self) -> String {
        self.pat.to_string()
    }

    pub fn ty_string(&self) -> String {
        self.ty.ty_str.clone()
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

struct UserDefinedStructReturnType {
    orig_type_path: syn::TypePath,
    return_type: syn::ReturnType,
}

/// Currently, only `Result::<SEXP>`, `Result<()>`, and Self are supported
pub enum SavvyFnReturnType {
    Sexp(syn::ReturnType),
    Unit(syn::ReturnType),
    UserDefinedStruct(UserDefinedStructReturnType),
}

impl SavvyFnReturnType {
    pub fn inner(&self) -> &syn::ReturnType {
        match self {
            SavvyFnReturnType::Sexp(ret_ty) => ret_ty,
            SavvyFnReturnType::Unit(ret_ty) => ret_ty,
            SavvyFnReturnType::UserDefinedStruct(ret_ty) => &ret_ty.return_type,
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
                            return Ok(SavvyFnReturnType::Sexp(ret_ty));
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
                                    Ok(SavvyFnReturnType::Unit(return_type.clone()))
                                } else {
                                    e
                                }
                            }
                            syn::Type::Path(type_path) => {
                                if !is_type_path_savvy_or_no_qualifier(type_path) {
                                    return e;
                                }

                                let last_path_seg = type_path.path.segments.last().unwrap();
                                if &last_path_seg.ident.to_string() == "Sexp" {
                                    Ok(SavvyFnReturnType::Sexp(return_type.clone()))
                                } else {
                                    let ret_ty: syn::ReturnType =
                                        parse_quote!(-> savvy::Result<savvy::Sexp>);
                                    Ok(SavvyFnReturnType::UserDefinedStruct(
                                        UserDefinedStructReturnType {
                                            orig_type_path: type_path.clone(),
                                            return_type: ret_ty,
                                        },
                                    ))
                                }
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
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::Sexp(_)));
        }

        let ok_cases2: &[syn::ReturnType] = &[
            parse_quote!(-> Result<()>),
            parse_quote!(-> savvy::Result<()>),
        ];

        for rt in ok_cases2 {
            let srt = get_savvy_return_type(rt);
            assert!(srt.is_ok());
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::Unit(_)));
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
