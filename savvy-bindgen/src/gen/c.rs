use crate::{ParsedResult, SavvyFn, SavvyFnType};

impl SavvyFn {
    pub fn get_c_args(&self) -> Vec<(String, String)> {
        let mut out: Vec<_> = self
            .args
            .iter()
            .map(|arg| {
                let pat = arg.pat_string();
                let ty = arg.to_c_type_string();
                (pat, ty)
            })
            .collect();

        // if it's a method, add `self__` arg
        if let SavvyFnType::Method(_) = &self.fn_type {
            out.insert(0, ("self__".to_string(), "SEXP".to_string()))
        }

        out
    }

    /// Generate C function signature
    fn to_c_function_for_header(&self) -> String {
        let fn_name = self.fn_name_outer();
        let args = self
            .get_c_args()
            .iter()
            .map(|(pat, ty)| format!("{ty} {pat}"))
            .collect::<Vec<String>>()
            .join(", ");

        format!("SEXP {fn_name}({args});")
    }

    /// Generate C function implementation
    fn to_c_function_impl(&self) -> String {
        let fn_name = self.fn_name_outer();
        let args = self.get_c_args();

        let args_sig = args
            .iter()
            .map(|(pat, ty)| format!("{ty} {pat}"))
            .collect::<Vec<String>>()
            .join(", ");

        let args_call = args
            .iter()
            .map(|(pat, _)| pat.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        format!(
            "SEXP {fn_name}_wrapper({args_sig}) {{
    SEXP res = {fn_name}({args_call});
    return handle_result(res);
}}
"
        )
    }

    /// Generate C function call entry
    fn to_c_function_call_entry(&self) -> String {
        let fn_name = self.fn_name_outer();
        let n_args = self.get_c_args().len();
        format!(r#"    {{"{fn_name}", (DL_FUNC) &{fn_name}_wrapper, {n_args}}},"#)
    }
}

pub fn generate_c_header_file(parsed_result: &ParsedResult) -> String {
    let bare_fns = parsed_result
        .bare_fns
        .iter()
        .map(|x| x.to_c_function_for_header())
        .collect::<Vec<String>>()
        .join("\n");

    let impls = parsed_result
        .impls
        .iter()
        .map(|x| {
            let fns = x
                .fns
                .iter()
                .map(|x| x.to_c_function_for_header())
                .collect::<Vec<String>>()
                .join("\n");

            format!(
                "\n// methods and associated functions for {}\n{}",
                x.ty, fns
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!("{bare_fns}\n{impls}")
}

fn generate_c_function_impl(fns: &[SavvyFn]) -> String {
    fns.iter()
        .map(|x| x.to_c_function_impl())
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_c_function_call_entry(fns: &[SavvyFn]) -> String {
    fns.iter()
        .map(|x| x.to_c_function_call_entry())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate_c_impl_file(parsed_result: &ParsedResult, pkg_name: &str) -> String {
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

    let c_fns_bare = generate_c_function_impl(parsed_result.bare_fns.as_slice());

    let c_fns_impl = parsed_result
        .impls
        .iter()
        .map(|x| {
            format!(
                "\n// methods and associated functions for {}\n{}",
                x.ty,
                generate_c_function_impl(x.fns.as_slice())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let call_entries_bare = generate_c_function_call_entry(parsed_result.bare_fns.as_slice());

    let call_entries_impl = parsed_result
        .impls
        .iter()
        .map(|x| {
            format!(
                "\n// methods and associated functions for {}\n{}",
                x.ty,
                generate_c_function_call_entry(x.fns.as_slice())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{common_part}
{c_fns_bare}
{c_fns_impl}

static const R_CallMethodDef CallEntries[] = {{
{call_entries_bare}
{call_entries_impl}
    {{NULL, NULL, 0}}
}};

void R_init_{pkg_name}(DllInfo *dll) {{
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}}
"
    )
}
