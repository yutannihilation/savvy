type DynError = Box<dyn std::error::Error>;
use std::env;

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
        // TODO: this works only on my Windows laptop...
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
        // pre-defined symbols
        .allowlist_var("R_NamesSymbol")
        .allowlist_var("R_ClassSymbol")
        .allowlist_var("R_DimSymbol")
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
        .allowlist_function("Rf_install")
        .allowlist_function("Rf_getAttrib")
        .allowlist_function("Rf_setAttrib")
        // Integer
        .allowlist_function("INTEGER")
        .allowlist_function("INTEGER_ELT")
        .allowlist_function("SET_INTEGER_ELT")
        .allowlist_function("Rf_ScalarInteger")
        .allowlist_function("Rf_isInteger")
        // Real
        .allowlist_function("REAL")
        .allowlist_function("REAL_ELT")
        .allowlist_function("SET_COMPLEX_ELT")
        .allowlist_function("Rf_ScalarReal")
        .allowlist_function("Rf_isReal")
        // Complex
        .allowlist_type("RComplex")
        .allowlist_function("COMPLEX")
        .allowlist_function("COMPLEX_ELT")
        .allowlist_function("SET_COMPLEX_ELT")
        .allowlist_function("Rf_ScalarComplex")
        .allowlist_function("Rf_isComplex")
        // Logical
        .allowlist_function("LOGICAL")
        .allowlist_function("LOGICAL_ELT")
        .allowlist_function("SET_LOGICAL_ELT")
        .allowlist_function("Rf_ScalarLogical")
        .allowlist_function("Rf_isLogical")
        // String and character
        .allowlist_function("STRING_ELT")
        .allowlist_function("SET_STRING_ELT")
        .allowlist_function("Rf_ScalarString")
        .allowlist_function("Rf_isString")
        .allowlist_function("R_CHAR")
        .allowlist_function("Rf_mkCharLenCE")
        // List
        .allowlist_function("VECTOR_ELT")
        .allowlist_function("SET_VECTOR_ELT")
        // External pointer
        .allowlist_function("R_ClearExternalPtr")
        .allowlist_function("R_ExternalPtrAddr")
        .allowlist_function("R_MakeExternalPtr")
        .allowlist_function("R_RegisterCFinalizerEx")
        // Pairlist
        .allowlist_function("Rf_cons")
        .allowlist_function("Rf_lcons")
        .allowlist_function("CAR")
        .allowlist_function("CDR")
        .allowlist_function("SETCAR")
        .allowlist_function("SETCDR")
        .allowlist_function("SET_TAG")
        // Function and Environment
        .allowlist_function("Rf_isFunction")
        .allowlist_function("Rf_isEnvironment")
        .allowlist_function("Rf_eval")
        .allowlist_var("R_GlobalEnv")
        // parse
        .allowlist_item("ParseStatus")
        .allowlist_function("R_ParseVector")
        .allowlist_function("R_compute_identical")
        // protection
        .allowlist_function("Rf_protect")
        .allowlist_function("Rf_unprotect")
        .allowlist_function("R_PreserveObject")
        // type
        .allowlist_function("Rf_type2char")
        .allowlist_function("TYPEOF")
        // error
        .allowlist_function("Rf_errorcall")
        // I/O
        .allowlist_function("Rprintf")
        .allowlist_function("REprintf")
        // misc
        .allowlist_type("DllInfo")
        .allowlist_function("Rf_duplicate")
        .allowlist_function("Rf_coerceVector");

    let builder = builder
        // ALTREP
        .allowlist_function("MARK_NOT_MUTABLE")
        .allowlist_function("ALTREP")
        .allowlist_function("ALTREP_CLASS")
        .allowlist_function("R_new_altrep")
        .allowlist_function("R_altrep_data1")
        .allowlist_function("R_altrep_data2")
        .allowlist_function("R_set_altrep_data1")
        .allowlist_function("R_set_altrep_data2")
        .allowlist_item("R_altrep_class_t")
        .allowlist_function("R_set_altrep_Coerce_method")
        .allowlist_function("R_set_altrep_Length_method")
        .allowlist_function("R_set_altrep_Inspect_method")
        .allowlist_function("R_set_altrep_Duplicate_method")
        .allowlist_function("R_set_altrep_DuplicateEX_method")
        .allowlist_function("R_set_altrep_Unserialize_method")
        .allowlist_function("R_set_altrep_UnserializeEX_method")
        .allowlist_function("R_set_altrep_Serialized_state_method")
        // ALTVEC
        .allowlist_function("R_set_altvec_Dataptr_method")
        .allowlist_function("R_set_altvec_Dataptr_or_null_method")
        // ALTINTEGER
        .allowlist_item("R_altinteger_Elt_method_t")
        .allowlist_item("R_altinteger_Max_method_t")
        .allowlist_item("R_altinteger_Min_method_t")
        .allowlist_item("R_altinteger_Sum_method_t")
        .allowlist_item("R_altinteger_No_NA_method_t")
        .allowlist_item("R_altinteger_Is_sorted_method_t")
        .allowlist_item("R_altinteger_Get_region_method_t")
        .allowlist_function("R_set_altinteger_Elt_method")
        .allowlist_function("R_set_altinteger_Max_method")
        .allowlist_function("R_set_altinteger_Min_method")
        .allowlist_function("R_set_altinteger_Sum_method")
        .allowlist_function("R_set_altinteger_No_NA_method")
        .allowlist_function("R_set_altinteger_Is_sorted_method")
        .allowlist_function("R_set_altinteger_Get_region_method")
        .allowlist_function("R_make_altinteger_class")
        // ALTREAL
        .allowlist_item("R_altreal_Elt_method_t")
        .allowlist_item("R_altreal_Max_method_t")
        .allowlist_item("R_altreal_Min_method_t")
        .allowlist_item("R_altreal_Sum_method_t")
        .allowlist_item("R_altreal_No_NA_method_t")
        .allowlist_item("R_altreal_Is_sorted_method_t")
        .allowlist_item("R_altreal_Get_region_method_t")
        .allowlist_function("R_set_altreal_Elt_method")
        .allowlist_function("R_set_altreal_Max_method")
        .allowlist_function("R_set_altreal_Min_method")
        .allowlist_function("R_set_altreal_Sum_method")
        .allowlist_function("R_set_altreal_No_NA_method")
        .allowlist_function("R_set_altreal_Is_sorted_method")
        .allowlist_function("R_set_altreal_Get_region_method")
        .allowlist_function("R_make_altreal_class");

    let bindings = builder.generate().expect("Unable to generate bindings");

    let stdout = Box::new(std::io::stdout());
    bindings.write(stdout).expect("Couldn't write bindings!");

    Ok(())
}
