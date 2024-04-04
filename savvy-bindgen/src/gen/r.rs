use std::collections::HashMap;

use quote::format_ident;

use crate::ir::SavvyMergedImpl;
use crate::{MergedResult, SavvyEnum, SavvyFn, SavvyFnType};

use crate::ir::savvy_fn::{SavvyFnReturnType, UserDefinedStructReturnType};

fn get_r_doc_comment(docs: &[String]) -> String {
    docs.iter()
        .map(|doc| format!("#'{doc}"))
        .collect::<Vec<String>>()
        .join("\n")
}

impl SavvyFn {
    pub fn fn_name_r(&self) -> syn::Ident {
        match self.get_self_ty_ident() {
            Some(ty) => {
                // Special convention. If the method name is "new", use type
                // itself.
                if self.fn_name.to_string().as_str() == "new" {
                    ty
                } else {
                    format_ident!("{}_{}", ty, self.fn_name)
                }
            }
            None => self.fn_name.clone(),
        }
    }

    fn get_r_args(&self) -> Vec<String> {
        let mut out: Vec<_> = self.args.iter().map(|arg| arg.pat_string()).collect();

        // if it's a method, add `self__` arg
        if matches!(&self.fn_type, SavvyFnType::Method { .. }) {
            out.insert(0, "self".to_string())
        }

        out
    }

    fn to_r_function(&self) -> String {
        let fn_name = self.fn_name_r();
        let fn_name_c = self.fn_name_outer();

        let doc_comments = get_r_doc_comment(self.docs.as_slice());

        let args = self
            .get_c_args()
            .iter()
            .map(|(pat, _)| pat.clone())
            .collect::<Vec<String>>();

        let mut args_call = args.clone();
        args_call.insert(0, format!("{fn_name_c}__impl"));

        let args = args.join(", ");
        let args_call = args_call.join(", ");

        let mut body_lines = self.get_extractions();

        // If the result is NULL, wrap it with invisible
        let fn_call = match &self.return_type {
            SavvyFnReturnType::Unit(_) => {
                format!("  invisible(.Call({args_call}))")
            }
            SavvyFnReturnType::Sexp(_) => format!("  .Call({args_call})"),
            SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                ty_str, ..
            }) => {
                format!("  .savvy_wrap_{ty_str}(.Call({args_call}))")
            }
        };
        body_lines.push(fn_call);

        let body = body_lines.join("\n");

        format!(
            "{doc_comments}
{fn_name} <- function({args}) {{
{body}
}}
"
        )
    }

    fn get_extractions(&self) -> Vec<String> {
        self.args
            .iter()
            .flat_map(|arg| {
                if !arg.is_user_defined_type() {
                    return None;
                }

                let r_var = arg.pat_string();
                let r_class = arg.ty_string();
                Some(format!(
                    r#"  {r_var} <- .savvy_extract_ptr({r_var}, "{r_class}")"#
                ))
            })
            .collect::<Vec<String>>()
    }
}

fn generate_r_impl_for_impl(
    i: &SavvyMergedImpl,
    ty: &str,
    enum_types: &HashMap<String, &SavvyEnum>,
) -> String {
    let mut associated_fns: Vec<&SavvyFn> = Vec::new();
    let mut method_fns: Vec<&SavvyFn> = Vec::new();
    let class_r = ty;

    for savvy_fn in &i.fns {
        match savvy_fn.fn_type {
            SavvyFnType::AssociatedFunction(_) => associated_fns.push(savvy_fn),
            SavvyFnType::Method { .. } => method_fns.push(savvy_fn),
            SavvyFnType::BareFunction => panic!("Something is wrong"),
        }
    }

    let closures = method_fns
        .iter()
        .map(|x| {
            let fn_name = x.fn_name_r();
            let fn_name_c = x.fn_name_outer();

            let mut args = x.get_r_args();
            // Remove self from arguments for R
            let args_r = args
                .clone()
                .into_iter()
                .filter(|e| *e != "self")
                .collect::<Vec<String>>()
                .join(", ");

            args.insert(0, format!("{fn_name_c}__impl"));
            let args_call = args.join(", ");

            let mut body_lines = x.get_extractions();

            let fn_call = match &x.return_type {
                SavvyFnReturnType::Sexp(_) => format!(".Call({args_call})"),
                // If the result is NULL, wrap it with invisible
                SavvyFnReturnType::Unit(_) => format!("invisible(.Call({args_call}))"),
                // If the result is an external pointer, wrap it with the
                // corresponding wraping function
                SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                    ty_str,
                    ..
                }) => {
                    format!("  .savvy_wrap_{ty_str}(.Call({args_call}))")
                }
            };
            body_lines.push(fn_call);

            let body = body_lines.join("\n");

            format!(
                "{fn_name} <- function(self) {{
  function({args_r}) {{
  {body}
  }}
}}
"
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let methods = method_fns
        .iter()
        .map(|o| format!("  e${} <- {}(ptr)", o.fn_name, o.fn_name_r()))
        .collect::<Vec<String>>()
        .join("\n");

    let wrap_fn_name = format!(".savvy_wrap_{}", class_r);

    let wrap_fn = format!(
        r#"{wrap_fn_name} <- function(ptr) {{
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  {methods}
  
  class(e) <- "{class_r}"
  e
}}
"#
    );

    let associated_fns = associated_fns
        .iter()
        .map(|x| {
            let fn_name = x.fn_name.clone();
            let fn_name_c = x.fn_name_outer();

            let mut args = x.get_r_args();

            let args_r = args.join(", ");

            args.insert(0, format!("{fn_name_c}__impl"));
            let args_call = args.join(", ");

            let mut body_lines = x.get_extractions();

            let fn_call = match &x.return_type {
                SavvyFnReturnType::Sexp(_) => format!(".Call({args_call})"),
                // If the result is NULL, wrap it with invisible
                SavvyFnReturnType::Unit(_) => format!("invisible(.Call({args_call}))"),
                // If the result is an external pointer, wrap it with the
                // corresponding wraping function
                SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                    ty_str,
                    ..
                }) => {
                    format!("  .savvy_wrap_{ty_str}(.Call({args_call}))")
                }
            };

            body_lines.push(fn_call);

            let body = body_lines.join("\n");

            format!(
                r#"{class_r}${fn_name} <- function({args_r}) {{
{body}
}}
"#
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let init = match enum_types.get(ty) {
        Some(e) => generate_r_impl_for_enum(e),
        None => format!("{class_r} <- new.env(parent = emptyenv())"),
    };

    let doc_comments = get_r_doc_comment(i.docs.as_slice());

    format!(
        "### wrapper functions for {class_r}

{closures}
{wrap_fn}

{doc_comments}
{init}

### associated functions for {class_r}

{associated_fns}
"
    )
}

fn generate_r_impl_for_enum(e: &SavvyEnum) -> String {
    let class_r = e.ty.to_string();

    let variants = e
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| format!(r#"{class_r}${v} <- .savvy_wrap_{class_r}({i}L)"#))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        "{class_r} <- new.env(parent = emptyenv())
{variants}
"
    )
}

pub fn generate_r_impl_file(result: &MergedResult, pkg_name: &str) -> String {
    let enum_types: HashMap<String, &SavvyEnum> =
        result.enums.iter().map(|e| (e.ty.to_string(), e)).collect();

    let r_fns = result
        .bare_fns
        .iter()
        .map(|x| x.to_r_function())
        .collect::<Vec<String>>()
        .join("\n");

    let r_impls = result
        .impls
        .iter()
        .map(|(ty, i)| generate_r_impl_for_impl(i, ty, &enum_types))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#' @useDynLib {pkg_name}, .registration = TRUE
#' @keywords internal
NULL

# Check class and extract the external pointer embedded in the environment
.savvy_extract_ptr <- function(e, class) {{
  if(inherits(e, class)) {{
    e$.ptr
  }} else {{
    msg <- paste0("Expected ", class, ", got ", class(e)[1])
    stop(msg, call. = FALSE)
  }}
}}

{r_fns}
{r_impls}
"#
    )
}
