type DynError = Box<dyn std::error::Error>;
use std::env;
use std::path::PathBuf;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("show") => show()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:


show            show bindgen-generated bindings
"
    )
}

fn show() -> Result<(), DynError> {
    println!("cargo:rerun-if-changed=wrapper.h");

    let builder = bindgen::Builder::default().header("wrapper.h").clang_args([
        format!("-I{}", "C:/Program Files/R/R-devel/include"),
        // format!("--target={target}"),
    ]);

    let builder = builder
        // Basic types and variables
        .allowlist_type("SEXP")
        .allowlist_type("cetype_t")
        .allowlist_type("R_xlen_t")
        .allowlist_type("Rboolean")
        //
        // SEXPTYPEs
        // cf. https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#SEXPTYPEs
        .allowlist_type("SEXPTYPE")
        .allowlist_var("NILSXP")
        .allowlist_var("SYMSXP")
        .allowlist_var("LISTSXP")
        .allowlist_var("CLOSXP")
        .allowlist_var("ENVSXP")
        .allowlist_var("PROMSXP")
        .allowlist_var("LANGSXP")
        .allowlist_var("SPECIALSXP")
        .allowlist_var("BUILTINSXP")
        .allowlist_var("CHARSXP")
        .allowlist_var("LGLSXP")
        .allowlist_var("INTSXP")
        .allowlist_var("REALSXP")
        .allowlist_var("CPLXSXP")
        .allowlist_var("STRSXP")
        .allowlist_var("DOTSXP")
        .allowlist_var("ANYSXP")
        .allowlist_var("VECSXP")
        .allowlist_var("EXPRSXP")
        .allowlist_var("BCODESXP")
        .allowlist_var("EXTPTRSXP")
        .allowlist_var("WEAKREFSXP")
        .allowlist_var("RAWSXP")
        .allowlist_var("OBJSXP")
        // NULL-related
        .allowlist_var("R_NilValue")
        // Missing values
        // - There's no "R_NaLogical" because the internal representation is i32
        .allowlist_var("R_NaInt")
        .allowlist_var("R_NaReal")
        .allowlist_var("R_NaString")
        .allowlist_function("R_IsNA")
        // Allocation and attributes
        .allowlist_function("Rf_xlength")
        .allowlist_function("Rf_allocVector")
        .allowlist_function("Rf_getAttrib")
        .allowlist_function("Rf_setAttrib")
        // Integer
        .allowlist_function("INTEGER")
        .allowlist_function("INTEGER_ELT")
        .allowlist_function("SET_INTEGER_ELT")
        .allowlist_function("Rf_ScalarInteger")
        // Real
        .allowlist_function("REAL")
        .allowlist_function("REAL_ELT")
        .allowlist_function("SET_REAL_ELT")
        .allowlist_function("Rf_ScalarReal")
        // Logical
        .allowlist_function("LOGICAL")
        .allowlist_function("LOGICAL_ELT")
        .allowlist_function("SET_LOGICAL_ELT")
        .allowlist_function("Rf_ScalarLogical")
        // String and character
        .allowlist_function("STRING_ELT")
        .allowlist_function("SET_STRING_ELT")
        .allowlist_function("Rf_ScalarString")
        .allowlist_function("R_CHAR")
        .allowlist_function("Rf_mkCharLenCE")
        // External pointer
        .allowlist_function("R_ClearExternalPtr")
        .allowlist_function("R_ExternalPtrAddr")
        .allowlist_function("R_MakeExternalPtr")
        .allowlist_function("R_RegisterCFinalizerEx")
        // Pairlist
        .allowlist_function("Rf_cons")
        .allowlist_function("CAR")
        .allowlist_function("CDR")
        .allowlist_function("SETCAR")
        .allowlist_function("SETCDR")
        .allowlist_function("SET_TAG")
        // protection
        .allowlist_function("Rf_protect")
        .allowlist_function("Rf_unprotect")
        .allowlist_function("R_PreserveObject")
        // error
        .allowlist_function("Rf_errorcall")
        // I/O
        .allowlist_function("Rprintf")
        .allowlist_function("REprintf");

    let bindings = builder.generate().expect("Unable to generate bindings");

    let stdout = Box::new(std::io::stdout());
    bindings.write(stdout).expect("Couldn't write bindings!");

    Ok(())
}
