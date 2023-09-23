use quote::format_ident;

use crate::{ParsedResult, SavvyFn, SavvyFnType, SavvyImpl};

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
        if let SavvyFnType::Method(_) = &self.fn_type {
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

        let body = if self.has_result {
            format!(".Call({args_call})")
        } else {
            // If the result is NULL, wrap it with invisible
            format!("invisible(.Call({args_call}))")
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

impl SavvyImpl {
    fn generate_r_impl_for_impl(&self) -> String {
        let mut ctors: Vec<&SavvyFn> = Vec::new();
        let mut others: Vec<&SavvyFn> = Vec::new();
        let class_r = self.ty.clone();

        for savvy_fn in &self.fns {
            match savvy_fn.fn_type {
                SavvyFnType::Constructor(_) => ctors.push(savvy_fn),
                _ => others.push(savvy_fn),
            }
        }

        // TODO: error if no ctor

        let closures = others
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

                let body = if x.has_result {
                    format!(".Call({args_call})")
                } else {
                    // If the result is NULL, wrap it with invisible
                    format!("invisible(.Call({args_call}))")
                };

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

        let builders = ctors
            .iter()
            .map(|x| {
                let fn_name = x.fn_name_r();
                let fn_name_c = x.fn_name_outer();

                let mut args = x.get_r_args();

                let args_r = args.join(", ");

                args.insert(0, format!("{fn_name_c}__impl"));
                let args_call = args.join(", ");

                let methods = others
                    .iter()
                    .map(|o| format!("  e${} <- {}(self)", o.fn_name, o.fn_name_r()))
                    .collect::<Vec<String>>()
                    .join("\n");

                format!(
                    r#"{fn_name} <- function({args_r}) {{
  e <- new.env(parent = emptyenv())
  self <- .Call({args_call})

{methods}

  class(e) <- "{class_r}"
  e
}}
"#
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let doc_comments = get_r_doc_comment(self.docs.as_slice());

        format!(
            "{doc_comments}
{builders}

{closures}
"
        )
    }
}

pub fn generate_r_impl_file(parsed_result: &ParsedResult, pkg_name: &str) -> String {
    let r_fns = parsed_result
        .bare_fns
        .iter()
        .map(|x| x.to_r_function())
        .collect::<Vec<String>>()
        .join("\n");

    let r_impls = parsed_result
        .impls
        .iter()
        .map(|x| x.generate_r_impl_for_impl())
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#' @useDynLib {pkg_name}, .registration = TRUE
#' @keywords internal
NULL

{r_fns}
{r_impls}
"#
    )
}
