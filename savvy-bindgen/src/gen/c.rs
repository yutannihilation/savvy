use crate::{MergedResult, SavvyFn, SavvyFnType};

impl SavvyFn {
    // The return value is (pat, ty)
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

        if matches!(&self.fn_type, SavvyFnType::Method { .. }) {
            out.insert(0, ("self__".to_string(), "SEXP".to_string()))
        }
        out
    }

    /// Generate C function signature
    fn to_c_function_for_header(&self) -> String {
        let fn_name = self.fn_name_c_header();
        let args = self.get_c_args();

        let args_sig = if args.is_empty() {
            "void".to_string()
        } else {
            args.iter()
                .map(|(pat, ty)| format!("{ty} {pat}"))
                .collect::<Vec<String>>()
                .join(", ")
        };

        format!("SEXP {fn_name}({args_sig});")
    }

    /// Generate C function implementation
    fn to_c_function_impl(&self) -> String {
        let fn_name_ffi = self.fn_name_c_header();
        let fn_name_c = self.fn_name_c_impl();
        let args = self.get_c_args();

        let (args_sig, args_call) = if args.is_empty() {
            ("void".to_string(), "".to_string())
        } else {
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

            (args_sig, args_call)
        };

        format!(
            "SEXP {fn_name_c}({args_sig}) {{
    SEXP res = {fn_name_ffi}({args_call});
    return handle_result(res);
}}
"
        )
    }

    /// Generate C function call entry
    fn to_c_function_call_entry(&self) -> String {
        let fn_name_c = self.fn_name_c_impl();
        let n_args = self.get_c_args().len();
        format!(r#"    {{"{fn_name_c}", (DL_FUNC) &{fn_name_c}, {n_args}}},"#)
    }
}

pub fn generate_c_header_file(result: &MergedResult) -> String {
    let bare_fns = result
        .bare_fns
        .iter()
        .map(|x| x.to_c_function_for_header())
        .collect::<Vec<String>>()
        .join("\n");

    let impls = result
        .impls
        .iter()
        .map(|(ty, i)| {
            let fns = i
                .fns
                .iter()
                .map(|x| x.to_c_function_for_header())
                .collect::<Vec<String>>()
                .join("\n");

            format!("\n// methods and associated functions for {}\n{}", ty, fns)
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
        // initializaion functions don't need the R interface
        .filter(|x| !matches!(x.fn_type, SavvyFnType::InitFunction))
        .map(|x| x.to_c_function_call_entry())
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_c_initialization(fns: &[SavvyFn]) -> String {
    fns.iter()
        .filter(|x| matches!(x.fn_type, SavvyFnType::InitFunction))
        .map(|x| format!("    {}(dll);", x.fn_name_c_impl()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate_c_impl_file(result: &MergedResult, pkg_name: &str) -> String {
    let common_part = r#"
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

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
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}
"#;

    let mut c_fns: Vec<String> = Vec::new();
    let mut call_entries: Vec<String> = Vec::new();

    c_fns.push(generate_c_function_impl(&result.bare_fns));
    call_entries.push(generate_c_function_call_entry(&result.bare_fns));

    for (_, i) in result.impls.iter() {
        c_fns.push(generate_c_function_impl(i.fns.as_slice()));
        call_entries.push(generate_c_function_call_entry(i.fns.as_slice()));
    }

    let c_fns = c_fns.join("\n");
    let call_entries = call_entries.join("\n");

    let initialization = generate_c_initialization(&result.bare_fns);

    format!(
        "{common_part}
{c_fns}

static const R_CallMethodDef CallEntries[] = {{
{call_entries}
    {{NULL, NULL, 0}}
}};

void R_init_{pkg_name}(DllInfo *dll) {{
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.
{initialization}
}}
"
    )
}
