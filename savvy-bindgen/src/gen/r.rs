use std::collections::HashMap;

use syn::ext::IdentExt;
use syn::Ident;

use crate::ir::SavvyMergedImpl;
use crate::utils::add_indent;
use crate::{MergedResult, SavvyEnum, SavvyFn, SavvyFnArg, SavvyFnType};

use crate::ir::savvy_fn::{SavvyFnReturnType, UserDefinedStructReturnType};

fn get_r_doc_comment(docs: &[String]) -> String {
    docs.iter()
        .map(|doc| format!("#'{doc}"))
        .collect::<Vec<String>>()
        .join("\n")
}

fn ident_to_str_r(x: &Ident) -> String {
    x.unraw().to_string()
}

impl SavvyEnum {
    pub fn class_name_r(&self) -> String {
        self.ty.unraw().to_string()
    }
}

impl SavvyFnArg {
    pub fn arg_name_r(&self) -> String {
        self.pat.unraw().to_string()
    }
}

impl SavvyFn {
    pub fn fn_name_r(&self) -> String {
        self.name_r(true)
    }

    pub fn field_name_r(&self) -> String {
        self.name_r(false)
    }

    fn name_r(&self, with_ty: bool) -> String {
        let fn_name = ident_to_str_r(&self.fn_name);

        if !with_ty {
            return fn_name;
        }

        match self.get_self_ty_ident() {
            Some(ty) => {
                let ty_name = ident_to_str_r(&ty);

                // Special convention. If the method name is "new", use type
                // itself.
                if fn_name.as_str() == "new" {
                    ty_name
                } else {
                    format!("{}_{}", ty, fn_name)
                }
            }
            None => fn_name,
        }
    }

    // The return value is (pat, default value)
    fn get_r_args(&self) -> Vec<(String, Option<String>)> {
        let mut out: Vec<_> = self
            .args
            .iter()
            .map(|arg| {
                let arg_name = arg.arg_name_r();
                let default_value = if arg.is_optional() {
                    Some("NULL".to_string())
                } else {
                    None
                };

                (arg_name, default_value)
            })
            .collect();

        // if it's a method, add `self__` arg
        if matches!(&self.fn_type, SavvyFnType::Method { .. }) {
            out.insert(0, ("self".to_string(), None))
        }

        out
    }

    fn get_r_args_for_signature(&self) -> Vec<String> {
        self.get_r_args()
            .iter()
            .map(|(arg_name, default_value)| {
                if let Some(value) = default_value {
                    format!("`{arg_name}` = {value}")
                } else {
                    format!("`{arg_name}`")
                }
            })
            .collect::<Vec<_>>()
    }

    fn get_r_args_for_call(&self) -> Vec<String> {
        self.get_r_args()
            .iter()
            .map(|(arg_name, _)| format!("`{arg_name}`"))
            .collect::<Vec<_>>()
    }

    fn to_r_function(&self) -> String {
        let fn_name = self.fn_name_r();
        let fn_name_c = self.fn_name_c_impl();

        let doc_comments = get_r_doc_comment(self.docs.as_slice());

        let args_sig = self.get_r_args_for_signature().join(", ");

        let mut args_call = self.get_r_args_for_call();
        args_call.insert(0, format!("{fn_name_c}"));
        let args_call = args_call.join(", ");

        let mut body_lines = self.get_extractions();

        // If the result is NULL, wrap it with invisible
        let fn_call = match &self.return_type {
            SavvyFnReturnType::Unit(_) => {
                format!("invisible(.Call({args_call}))")
            }
            SavvyFnReturnType::Sexp(_) => format!(".Call({args_call})"),
            SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType { ty, .. }) => {
                let ty_str = ident_to_str_r(ty);
                format!(".savvy_wrap_{ty_str}(.Call({args_call}))")
            }
        };
        body_lines.push(fn_call);

        let body = add_indent(&body_lines.join("\n"), 2);

        format!(
            "{doc_comments}
`{fn_name}` <- function({args_sig}) {{
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

                let r_var = arg.arg_name_r();
                let r_class = arg.ty_string();
                Some(format!(
                    r#"`{r_var}` <- .savvy_extract_ptr(`{r_var}`, "{r_class}")"#
                ))
            })
            .collect::<Vec<String>>()
    }
}

fn generate_r_impl_for_impl(
    i: &SavvyMergedImpl,
    ty: &Ident,
    enum_types: &HashMap<Ident, &SavvyEnum>,
) -> String {
    let mut associated_fns: Vec<&SavvyFn> = Vec::new();
    let mut method_fns: Vec<&SavvyFn> = Vec::new();
    let class_r = ident_to_str_r(ty);
    let class_r_for_bundle = format!("{class_r}__bundle");

    for savvy_fn in &i.fns {
        match savvy_fn.fn_type {
            SavvyFnType::AssociatedFunction(_) => associated_fns.push(savvy_fn),
            SavvyFnType::Method { .. } => method_fns.push(savvy_fn),
            _ => panic!("Something is wrong"),
        }
    }

    let closures = method_fns
        .iter()
        .map(|x| {
            let fn_name = x.fn_name_r();
            let fn_name_c = x.fn_name_c_impl();

            // Remove self from arguments for R
            let args_sig = x
                .get_r_args_for_signature()
                .into_iter()
                .filter(|e| *e != "`self`")
                .collect::<Vec<String>>()
                .join(", ");

            let mut args_call = x.get_r_args_for_call();
            args_call.insert(0, format!("{fn_name_c}"));
            let args_call = args_call.join(", ");

            let mut body_lines = x.get_extractions();

            let fn_call = match &x.return_type {
                SavvyFnReturnType::Sexp(_) => format!(".Call({args_call})"),
                // If the result is NULL, wrap it with invisible
                SavvyFnReturnType::Unit(_) => format!("invisible(.Call({args_call}))"),
                // If the result is an external pointer, wrap it with the
                // corresponding wraping function
                SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                    ty, ..
                }) => {
                    let ty_str = ident_to_str_r(ty);
                    format!(".savvy_wrap_{ty_str}(.Call({args_call}))")
                }
            };
            body_lines.push(fn_call);

            let body = add_indent(&body_lines.join("\n"), 4);

            format!(
                "`{fn_name}` <- function(self) {{
  function({args_sig}) {{
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
        .map(|o| format!("  e$`{}` <- `{}`(ptr)", o.field_name_r(), o.fn_name_r()))
        .collect::<Vec<String>>()
        .join("\n");

    let wrap_fn_name = format!(".savvy_wrap_{}", class_r);

    let wrap_fn = format!(
        r#"`{wrap_fn_name}` <- function(ptr) {{
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
{methods}

  class(e) <- "{class_r}"
  e
}}

#' @export
`$<-.{class_r}` <- function(x, name, value) stop("{class_r} cannot be modified", call. = FALSE)

#' @export
`[[<-.{class_r}` <- function(x, i, value) stop("{class_r} cannot be modified", call. = FALSE)
"#
    );

    let associated_fns = associated_fns
        .iter()
        .map(|x| {
            let fn_name = x.field_name_r();
            let fn_name_c = x.fn_name_c_impl();

            let args_sig = x.get_r_args_for_signature().join(", ");

            let mut args_call = x.get_r_args_for_call();
            args_call.insert(0, format!("{fn_name_c}"));
            let args_call = args_call.join(", ");

            let mut body_lines = x.get_extractions();

            let fn_call = match &x.return_type {
                SavvyFnReturnType::Sexp(_) => format!(".Call({args_call})"),
                // If the result is NULL, wrap it with invisible
                SavvyFnReturnType::Unit(_) => format!("invisible(.Call({args_call}))"),
                // If the result is an external pointer, wrap it with the
                // corresponding wraping function
                SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                    ty, ..
                }) => {
                    let ty_str = ident_to_str_r(ty);
                    format!(".savvy_wrap_{ty_str}(.Call({args_call}))")
                }
            };

            body_lines.push(fn_call);

            let body = add_indent(&body_lines.join("\n"), 2);

            format!(
                r#"`{class_r}`$`{fn_name}` <- function({args_sig}) {{
{body}
}}
"#
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let init = match enum_types.get(ty) {
        Some(e) => generate_r_impl_for_enum(e),
        None => format!("`{class_r}` <- new.env(parent = emptyenv())"),
    };

    let doc_comments = get_r_doc_comment(i.docs.as_slice());

    format!(
        r#"### wrapper functions for {class_r}

{closures}
{wrap_fn}

{doc_comments}
{init}

#' @export
`$<-.{class_r}` <- function(x, name, value) stop("{class_r} cannot be modified", call. = FALSE)

#' @export
`[[<-.{class_r}` <- function(x, i, value) stop("{class_r} cannot be modified", call. = FALSE)

### associated functions for {class_r}

{associated_fns}

class(`{class_r}`) <- "{class_r_for_bundle}"

#' @export
`print.{class_r_for_bundle}` <- function(x, ...) {{
  cat('{class_r}')
}}

#' @export
`$<-.{class_r_for_bundle}` <- function(x, name, value) stop("{class_r} cannot be modified", call. = FALSE)

#' @export
`[[<-.{class_r_for_bundle}` <- function(x, i, value) stop("{class_r} cannot be modified", call. = FALSE)
"#
    )
}

fn generate_r_impl_for_enum(e: &SavvyEnum) -> String {
    let class_r = e.class_name_r();
    let class_r_for_bundle = format!("{class_r}__bundle");

    let variants = e
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let v = ident_to_str_r(v);
            format!(r#"`{class_r}`$`{v}` <- .savvy_wrap_{class_r}({i}L)"#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let variant_labels = e
        .variants
        .iter()
        .map(|x| format!(r#""{}""#, ident_to_str_r(x)))
        .collect::<Vec<String>>()
        .join(", ");

    format!(
        r#"`{class_r}` <- new.env(parent = emptyenv())
{variants}

#' @export
`$.{class_r_for_bundle}` <- function(x, name) {{
  if (!name %in% c({variant_labels})) {{
    stop(paste0("Unknown variant: ", name), call. = FALSE)
  }}

  NextMethod()
}}

#' @export
`[[.{class_r_for_bundle}` <- function(x, i) {{
  if (is.numeric(i)) {{
    stop("{class_r} cannot be subset by index", call. = FALSE)
  }}

  if (!i %in% c({variant_labels})) {{
    stop(paste0("Unknown variant: ", i), call. = FALSE)
  }}

  NextMethod()
}}

#' @export
`print.{class_r}` <- function(x, ...) {{
  idx <- x$.ptr + 1L
  label <- c({variant_labels})[idx]
  if (is.na(label)) {{
    stop("Unexpected value for {class_r}", call. = TRUE)
  }}
  cat("{class_r}::", label, sep = "")
}}
"#
    )
}

pub fn generate_r_impl_file(result: &MergedResult, pkg_name: &str) -> String {
    let enum_types: HashMap<Ident, &SavvyEnum> =
        result.enums.iter().map(|e| (e.ty.clone(), e)).collect();

    let r_fns = result
        .bare_fns
        .iter()
        // initializaion functions don't need the R interface
        .filter(|x| !matches!(x.fn_type, SavvyFnType::InitFunction))
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
        r#"# Generated by savvy: do not edit by hand
#
# Note:
#   This wrapper file is named as `000-wrappers.R` so that this file is loaded
#   first, which allows users to override the functions defined here (e.g., a
#   print() method for an enum).

#' @useDynLib {pkg_name}, .registration = TRUE
#' @keywords internal
NULL

# Check class and extract the external pointer embedded in the environment
.savvy_extract_ptr <- function(e, class) {{
  if(is.null(e)) {{
    return(NULL)
  }}

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
