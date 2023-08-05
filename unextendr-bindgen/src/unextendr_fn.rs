use quote::format_ident;
use syn::{parse_quote, FnArg::Typed, Item, Pat::Ident, PatType, Stmt};

pub struct UnextendrFn {
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
                    // Use only the last part to support both the qualified type
                    // path (e.g., `unextendr::IntegerSxp`), and single type
                    // (e.g., `IntegerSxp`)
                    syn::Type::Path(type_path) => &type_path.path.segments.last().unwrap().ident,
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

    pub fn make_inner_fn(&self) -> syn::ItemFn {
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

    pub fn make_outer_fn(&self) -> syn::ItemFn {
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

pub(crate) trait ToSourceCode {
    fn to_c_function_for_header(&self) -> String;
    fn to_c_function_for_init(&self) -> String;
    fn to_r_function(&self) -> String;
}

impl ToSourceCode for UnextendrFn {
    fn to_c_function_for_header(&self) -> String {
        let fn_name = self.fn_name_outer();
        let args = self
            .args_new
            .iter()
            .map(|arg| {
                if let Typed(PatType { pat, ty, .. }) = arg {
                    let pat_ident = match pat.as_ref() {
                        syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                        _ => panic!("Unsupported signature"),
                    };

                    // TODO: Currently, only SEXP can be accepted
                    let ty_ident = match ty.as_ref() {
                        syn::Type::Path(path) => {
                            let last_ty = &path.path.segments.last().unwrap().ident;
                            match last_ty.to_string().as_str() {
                                "SEXP" => "SEXP",
                                _ => panic!("Unsupported signature"),
                            }
                        }
                        _ => panic!("Unsupported signature"),
                    };

                    format!("{ty_ident} {pat_ident}")
                } else {
                    panic!("Unsupported signature")
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("SEXP {fn_name}({args});")
    }

    fn to_c_function_for_init(&self) -> String {
        "".into()
    }

    fn to_r_function(&self) -> String {
        "".into()
    }
}

impl ToSourceCode for Vec<UnextendrFn> {
    fn to_c_function_for_header(&self) -> String {
        self.iter()
            .map(|x| x.to_c_function_for_header())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn to_c_function_for_init(&self) -> String {
        "".into()
    }

    fn to_r_function(&self) -> String {
        "".into()
    }
}
