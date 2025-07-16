use proc_macro2::Span;
use quote::format_ident;
use syn::{ext::IdentExt, parse_quote, Attribute, FnArg::Typed, Pat::Ident, PatType, Signature, Stmt};

use crate::utils::extract_docs;

#[derive(Clone, PartialEq)]
enum SavvyInputTypeCategory {
    Sexp,
    SexpWrapper,
    PrimitiveType,
    UserDefinedTypeRef, // &T
    UserDefinedType,    // T
    DllInfo,
}

#[derive(Clone)]
struct SavvyInputType {
    category: SavvyInputTypeCategory,
    ty_orig: syn::Type,
    ty_str: String,
    optional: bool,
}

#[allow(dead_code)]
impl SavvyInputType {
    fn from_type(ty: &syn::Type, in_option: bool) -> syn::Result<Self> {
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
                            optional: in_option,
                        })
                    } else {
                        Ok(Self {
                            category: SavvyInputTypeCategory::UserDefinedTypeRef,
                            ty_orig: ty.clone(),
                            ty_str,
                            optional: in_option,
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
                let type_path_last = type_path.path.segments.last().unwrap();
                let type_ident = &type_path_last.ident;
                let ty_str = type_ident.to_string();
                match ty_str.as_str() {
                    "Option" => {
                        if in_option {
                            return Err(syn::Error::new_spanned(
                                type_path,
                                "`Option` cannot be nested",
                            ));
                        }

                        if let syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments { args, .. },
                        ) = &type_path_last.arguments
                        {
                            if args.len() == 1 {
                                if let syn::GenericArgument::Type(ty) = &args.first().unwrap() {
                                    return Self::from_type(ty, true);
                                }
                            }
                        }

                        Err(syn::Error::new_spanned(
                            type_path,
                            "Option<T> can accept only a type",
                        ))
                    }

                    // Owned-types are not allowed for the input
                    "OwnedIntegerSexp" | "OwnedRealSexp" | "OwnedComplexSexp"
                    | "OwnedLogicalSexp" | "OwnedRawSexp" | "OwnedStringSexp" | "OwnedListSexp" => {
                        let msg = format!(
                            "`Owned-` types are not allowed here. Did you mean `{}`?",
                            ty_str.strip_prefix("Owned").unwrap()
                        );
                        Err(syn::Error::new_spanned(type_path, msg))
                    }

                    // Since Sexp doesn't need to be converted by try_from(),
                    // this needs to be handled separately.
                    "Sexp" => Ok(Self {
                        category: SavvyInputTypeCategory::Sexp,
                        ty_orig: ty.clone(),
                        ty_str,
                        optional: in_option,
                    }),

                    // Read-only types
                    "IntegerSexp" | "RealSexp" | "NumericSexp" | "ComplexSexp"
                    | "LogicalSexp" | "RawSexp" | "StringSexp" | "ListSexp" | "FunctionSexp"
                    | "EnvironmentSexp" => Ok(Self {
                        category: SavvyInputTypeCategory::SexpWrapper,
                        ty_orig: ty.clone(),
                        ty_str,
                        optional: in_option,
                    }),

                    // Primitive types
                    "i32" | "usize" | "f64" | "bool" | "u8" | "NumericScalar" => Ok(Self {
                        category: SavvyInputTypeCategory::PrimitiveType,
                        ty_orig: ty.clone(),
                        ty_str,
                        optional: in_option,
                    }),

                    "DllInfo" => Err(syn::Error::new_spanned(
                        type_path,
                        "DllInfo must be `*mut DllInfo`",
                    )),

                    _ => Ok(Self {
                        category: SavvyInputTypeCategory::UserDefinedType,
                        ty_orig: ty.clone(),
                        ty_str,
                        optional: in_option,
                    }),
                }
            }

            // Only *mut DllInfo falls here
            syn::Type::Ptr(syn::TypePtr {
                mutability, elem, ..
            }) => {
                let type_ident = if let syn::Type::Path(p) = elem.as_ref() {
                    p.path.segments.last().unwrap().ident.to_string()
                } else {
                    "".to_string()
                };

                if &type_ident != "DllInfo" {
                    return Err(syn::Error::new_spanned(
                        ty.clone(),
                        "Unexpected type specification: {:?}",
                    ));
                }

                if mutability.is_none() {
                    return Err(syn::Error::new_spanned(
                        ty.clone(),
                        "DllInfo must be `*mut DllInfo`",
                    ));
                }

                Ok(Self {
                    category: SavvyInputTypeCategory::DllInfo,
                    ty_orig: ty.clone(),
                    ty_str: type_ident.to_string(),
                    optional: in_option,
                })
            }

            _ => Err(syn::Error::new_spanned(
                ty.clone(),
                "Unexpected type specification: {:?}",
            )),
        }
    }

    /// Returns the corresponding type for internal function.
    fn to_rust_type_outer(&self) -> syn::Type {
        self.ty_orig.clone()
    }

    /// Returns the corresponding type for API function.
    fn to_rust_type_inner(&self) -> syn::Type {
        if matches!(self.category, SavvyInputTypeCategory::DllInfo) {
            self.ty_orig.clone()
        } else {
            parse_quote!(savvy::ffi::SEXP)
        }
    }

    /// Returns the corresponding type for C function.
    fn to_c_type(&self) -> String {
        if matches!(self.category, SavvyInputTypeCategory::DllInfo) {
            "DllInfo*".to_string()
        } else {
            "SEXP".to_string()
        }
    }
}

#[derive(Clone)]
pub struct SavvyFnArg {
    pub(crate) pat: syn::Ident,
    ty: SavvyInputType,
}

impl SavvyFnArg {
    pub fn pat(&self) -> syn::Ident {
        self.pat.clone()
    }

    pub fn is_user_defined_type(&self) -> bool {
        matches!(
            &self.ty.category,
            SavvyInputTypeCategory::UserDefinedTypeRef | SavvyInputTypeCategory::UserDefinedType
        )
    }

    pub fn ty_string(&self) -> String {
        self.ty.ty_str.clone()
    }

    pub fn is_optional(&self) -> bool {
        self.ty.optional
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

impl PartialEq for SavvyFnArg {
    fn eq(&self, other: &Self) -> bool {
        self.pat == other.pat && self.ty.category == other.ty.category && self.ty.ty_str == other.ty.ty_str && self.ty.optional == other.ty.optional
    }
}

/// Return type of a user-defined struct. This can be either
///
/// - `savvy::Result<Foo>`
/// - `savvy::Result<Self>`
/// - `Self`
#[derive(Clone)]
pub struct UserDefinedStructReturnType {
    pub(crate) ty: syn::Ident,
    pub(crate) return_type: syn::ReturnType,
    pub(crate) wrapped_with_result: bool,
}

/// Return type. This can be either
///
/// - `savvy::Result<Sexp>`
/// - `savvy::Result<()>`
/// - a user-defined struct
///     - `savvy::Result<Foo>`
///     - `savvy::Result<Self>`
///     - `Self`
#[derive(Clone)]
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

#[derive(Clone)]
pub enum SavvyFnType {
    /// A function that doesn't belong to a struct
    BareFunction,
    /// A function that belongs to a struct, and the first argument is `self`.
    /// Contains the type name of the sturct.
    Method {
        ty: syn::Type,
        reference: bool,
        mutability: bool,
    },
    /// A function that belongs to a struct, but  the first argument is not
    /// `self`. Contains the type name of the sturct.
    AssociatedFunction(syn::Type),
    /// A function to be executed in the package's initialization routine.
    InitFunction,
}

#[derive(Clone)]
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
    /// Additional lines to convert `SEXP` to the specific types
    pub stmts_additional: Vec<syn::Stmt>,
}

#[allow(dead_code)]
impl SavvyFn {
    pub(crate) fn get_self_ty_ident(&self) -> Option<syn::Ident> {
        let self_ty = match &self.fn_type {
            SavvyFnType::Method { ty, .. } => ty,
            SavvyFnType::AssociatedFunction(ty) => ty,
            _ => return None,
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

    /// Returns a function name to be exported from Rust to C
    pub fn fn_name_c_header(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("savvy_{}_{}__ffi", ty, self.fn_name),
            None => format_ident!("savvy_{}__ffi", self.fn_name),
        }
    }

    /// Returns a function name to be exported from C to R
    pub fn fn_name_c_impl(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => format_ident!("savvy_{}_{}__impl", ty, self.fn_name),
            None => format_ident!("savvy_{}__impl", self.fn_name),
        }
    }

    pub fn from_fn(orig: &syn::ItemFn, as_init_fn: bool) -> syn::Result<Self> {
        let fn_type = if as_init_fn {
            SavvyFnType::InitFunction
        } else {
            SavvyFnType::BareFunction
        };
        Self::new(&orig.attrs, &orig.sig, fn_type, None)
    }

    pub fn from_impl_fn(
        orig: &syn::ImplItemFn,
        fn_type: SavvyFnType,
        self_ty: &syn::Type,
    ) -> syn::Result<Self> {
        Self::new(&orig.attrs, &orig.sig, fn_type, Some(self_ty))
    }

    pub fn new(
        attrs: &[Attribute],
        sig: &Signature,
        fn_type: SavvyFnType,
        self_ty: Option<&syn::Type>,
    ) -> syn::Result<Self> {
        // TODO: check function signature and abort if any of it is unexpected one.

        let mut attrs = attrs.to_vec();
        // Remove #[savvy] and #[savvy_init]
        attrs.retain(|attr| {
            !(attr == &parse_quote!(#[savvy]) || attr == &parse_quote!(#[savvy_init]))
        });

        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());

        let fn_name = sig.ident.clone();

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

                    let ty = match SavvyInputType::from_type(ty.as_ref(), false) {
                        Ok(ty) => ty,
                        Err(e) => return Some(Err(e)),
                    };
                    let ty_ident = ty.to_rust_type_outer();

                    match (&fn_type, &ty.category) {
                        // DllInfo is passed as it is
                        (&SavvyFnType::InitFunction, &SavvyInputTypeCategory::DllInfo) => {}
                        
                        (&SavvyFnType::InitFunction, _) => {
                            return Some(Err(syn::Error::new_spanned(
                                ty.ty_orig,
                                "#[savvy_init] can be used only on a function that takes `*mut DllInfo`",
                            )));
                        }

                        (_, &SavvyInputTypeCategory::DllInfo) => {
                                return Some(Err(syn::Error::new_spanned(
                                    ty.ty_orig,
                                    "#[savvy] doesn't accept `*mut DllInfo`. Did you mean #[savvy_init]?",
                                )));
                        }

                        (_, &SavvyInputTypeCategory::Sexp) => {
                            if ty.optional {
                                stmts_additional.push(parse_quote! { let #pat = savvy::Sexp(#pat); });
                                stmts_additional.push(parse_quote! { 
                                    let #pat = if #pat.is_null() { 
                                        None
                                     } else { 
                                        Some(#pat)
                                    };
                                })
                            } else {
                                stmts_additional.push(parse_quote! {
                                    let #pat = savvy::Sexp(#pat);
                                });
                            }
                        }

                        (_, _) => {
                            let arg_lit = syn::LitStr::new(&pat.unraw().to_string(), Span::call_site());
                            if ty.optional {
                                stmts_additional.push(parse_quote! { let #pat = savvy::Sexp(#pat); });
                                stmts_additional.push(parse_quote! { 
                                    let #pat = if #pat.is_null() { 
                                        None
                                     } else { 
                                        Some(<#ty_ident>::try_from(#pat).map_err(|e| e.with_arg_name(#arg_lit))?) 
                                    };
                                })
                            } else {
                                stmts_additional.push(parse_quote! {
                                    let #pat = <#ty_ident>::try_from(savvy::Sexp(#pat)).map_err(|e| e.with_arg_name(#arg_lit))?;
                                });
                            }
                        }
                    }

                    Some(Ok(SavvyFnArg { pat, ty }))
                }
                // Skip `self`
                syn::FnArg::Receiver(syn::Receiver { .. }) => None,
            })
            .collect::<syn::Result<Vec<SavvyFnArg>>>()?;

        // reject signature like fn (x: Option<i32>, y: i32)
        let mut args_after_optional = args_new.iter().skip_while(|x| !x.is_optional());
        if args_after_optional.any(|x| !x.is_optional()) {
            return Err(syn::Error::new_spanned(
                sig.inputs.clone(),
                "optional args can be placed only after mandatory args",
            ));
        }

        // Check for init function
        let is_init_fn = args_new
            .iter()
            .any(|x| matches!(x.ty.category, SavvyInputTypeCategory::DllInfo));
        if is_init_fn && args_new.len() > 1 {
            return Err(syn::Error::new_spanned(
                sig,
                "Initialization function can accept `*mut DllInfo` only",
            ));
        }

        let fn_type = if is_init_fn {
            SavvyFnType::InitFunction
        } else {
            fn_type
        };

        Ok(Self {
            docs,
            attrs,
            fn_name,
            fn_type,
            args: args_new,
            return_type: get_savvy_return_type(&sig.output, self_ty)?,
            stmts_additional,
        })
    }
}

fn self_ty_to_actual_ty(self_ty: Option<&syn::Type>) -> Option<syn::Ident> {
    if let Some(syn::Type::Path(type_path)) = self_ty {
        Some(type_path.path.segments.last().unwrap().ident.clone())
    } else {
        None
    }
}

// Allowed return types are the followings. Note that, `Self` is converted to
// `EXTPTRSXP`, so the same as `savvy::Result<savvy::Sexp>`.
//
// - `savvy::Result<savvy::Sexp>`
// - `savvy::Result<()>`
// - `Self`
fn get_savvy_return_type(
    return_type: &syn::ReturnType,
    self_ty: Option<&syn::Type>,
) -> syn::Result<SavvyFnReturnType> {
    match return_type {
        syn::ReturnType::Default => Err(syn::Error::new_spanned(
            return_type.clone(),
            "function must have return type",
        )),

        syn::ReturnType::Type(_, ty) => {
            let e = Err(syn::Error::new_spanned(
                return_type.clone(),
                "the return type must be savvy::Result<T> or savvy::Result<()>",
            ));

            // Check if the type path is savvy::Result<..> or Result<..> and get
            // the arguments inside < >.
            let path_args = match ty.as_ref() {
                syn::Type::Path(type_path) => {
                    // At least it must be savvy::T or T
                    if !is_type_path_savvy_or_no_qualifier(type_path) {
                        return e;
                    }

                    let last_path_seg = type_path.path.segments.last().unwrap();
                    match (
                        last_path_seg.ident.to_string().as_str(),
                        self_ty_to_actual_ty(self_ty),
                    ) {
                        // if Result, do further investigation about hte inside.
                        ("Result", _) => {}
                        // if Self or the same as the self type, it's allowed
                        (ret_ty_str, Some(ty_actual)) => {
                            if ret_ty_str != "Self" && ty_actual != ret_ty_str {
                                return e;
                            } else {
                                return Ok(SavvyFnReturnType::UserDefinedStruct(
                                    UserDefinedStructReturnType {
                                        ty: ty_actual,
                                        return_type: parse_quote!(-> savvy::Result<#self_ty>),
                                        wrapped_with_result: false,
                                    },
                                ));
                            }
                        }
                        _ => {
                            return e;
                        }
                    }
                    &last_path_seg.arguments
                }
                _ => return e,
            };

            // Check `T`` in savvy::Result<T>

            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) = path_args
            {
                if args.len() != 1 {
                    return e;
                }

                if let syn::GenericArgument::Type(ty) = &args.first().unwrap() {
                    match ty {
                        syn::Type::Tuple(type_tuple) => {
                            if type_tuple.elems.is_empty() {
                                return Ok(SavvyFnReturnType::Unit(return_type.clone()));
                            }
                        }

                        syn::Type::Path(type_path) => {
                            let last_ident = &type_path.path.segments.last().unwrap().ident;
                            match last_ident.to_string().as_str() {
                                "Sexp" => return Ok(SavvyFnReturnType::Sexp(return_type.clone())),

                                // if it's `savvy::Result<Self>`, replace `Self` with the actual type
                                "Self" => {
                                    if let Some(ty_actual) = self_ty_to_actual_ty(self_ty) {
                                        return Ok(SavvyFnReturnType::UserDefinedStruct(
                                            UserDefinedStructReturnType {
                                                ty: ty_actual,
                                                return_type: parse_quote!(-> savvy::Result<#self_ty>),
                                                wrapped_with_result: true,
                                            },
                                        ));
                                    }
                                }

                                // catch common mistakes
                                wrong_ty @ ("String" | "i32" | "usize" | "f64" | "bool") => {
                                    let msg = format!(
"Return type must be either (), savvy::Sexp, or a user-defined type.
You can use .try_into() to convert {wrong_ty} to savvy::Sexp."
                                    );
                                    return Err(syn::Error::new_spanned(type_path, msg));
                                }

                                // if it's the actual type, use it as it is.
                                _ => {
                                    return Ok(SavvyFnReturnType::UserDefinedStruct(
                                        UserDefinedStructReturnType {
                                            ty: last_ident.clone(),
                                            return_type: return_type.clone(),
                                            wrapped_with_result: true,
                                        },
                                    ))
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }

            e
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
    fn test_detect_return_type_sexp() {
        let ok_cases1: &[syn::ReturnType] = &[
            parse_quote!(-> Result<Sexp>),
            parse_quote!(-> savvy::Result<Sexp>),
            parse_quote!(-> savvy::Result<savvy::Sexp>),
        ];

        for rt in ok_cases1 {
            let srt = get_savvy_return_type(rt, None);
            assert!(srt.is_ok());
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::Sexp(_)));
        }
    }

    #[test]
    fn test_detect_return_type_unit() {
        let ok_cases2: &[syn::ReturnType] = &[
            parse_quote!(-> Result<()>),
            parse_quote!(-> savvy::Result<()>),
        ];

        for rt in ok_cases2 {
            let srt = get_savvy_return_type(rt, None);
            assert!(srt.is_ok());
            assert!(matches!(srt.unwrap(), SavvyFnReturnType::Unit(_)));
        }
    }

    #[test]
    fn test_detect_return_type_sturct() {
        let ok_cases3: &[syn::ReturnType] = &[
            parse_quote!(-> Result<Foo>),
            parse_quote!(-> savvy::Result<Foo>),
        ];

        for rt in ok_cases3 {
            let srt = get_savvy_return_type(rt, None);
            assert!(srt.is_ok());
            assert!(matches!(
                srt.unwrap(),
                SavvyFnReturnType::UserDefinedStruct(_)
            ));
        }
    }

    #[test]
    fn test_detect_return_type_self() {
        let ok_cases4: &[syn::ReturnType] = &[
            parse_quote!(-> Self),
            parse_quote!(-> Result<Self>),
            parse_quote!(-> savvy::Result<Self>),
        ];
        let self_ty: syn::Type = parse_quote!(Foo);

        for (i, rt) in ok_cases4.iter().enumerate() {
            let srt = get_savvy_return_type(rt, Some(&self_ty));
            assert!(srt.is_ok());
            if let SavvyFnReturnType::UserDefinedStruct(uds) = srt.unwrap() {
                assert_eq!(uds.ty.to_string().as_str(), "Foo");
                assert_eq!(uds.return_type, parse_quote!(-> savvy::Result<Foo>));
                assert_eq!(uds.wrapped_with_result, i != 0); // only the first case is false
            } else {
                panic!("Unpexpected SavvyFnReturnType");
            }
        }
    }

    #[test]
    fn test_detect_return_type_fail() {
        let err_cases: &[syn::ReturnType] = &[
            parse_quote!(-> Foo),
            parse_quote!(-> savvy::Result<(T, T)>),
            parse_quote!(-> foo::Result<Sexp>),
            parse_quote!(),
        ];

        for rt in err_cases {
            assert!(get_savvy_return_type(rt, None).is_err())
        }
    }
}
