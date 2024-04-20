#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// internal types

pub type R_xlen_t = isize;

pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_int;

#[cfg(feature = "complex")]
pub use num_complex::Complex64;

// SEXP
pub type SEXP = *mut ::std::os::raw::c_void;

// SEXPTYPE

pub type SEXPTYPE = ::std::os::raw::c_uint;

pub const NILSXP: SEXPTYPE = 0;
pub const SYMSXP: SEXPTYPE = 1;
pub const LISTSXP: SEXPTYPE = 2;
pub const CLOSXP: SEXPTYPE = 3;
pub const ENVSXP: SEXPTYPE = 4;
pub const PROMSXP: SEXPTYPE = 5;
pub const LANGSXP: SEXPTYPE = 6;
pub const SPECIALSXP: SEXPTYPE = 7;
pub const BUILTINSXP: SEXPTYPE = 8;
pub const CHARSXP: SEXPTYPE = 9;
pub const LGLSXP: SEXPTYPE = 10;
pub const INTSXP: SEXPTYPE = 13;
pub const REALSXP: SEXPTYPE = 14;
pub const CPLXSXP: SEXPTYPE = 15;
pub const STRSXP: SEXPTYPE = 16;
pub const DOTSXP: SEXPTYPE = 17;
pub const ANYSXP: SEXPTYPE = 18;
pub const VECSXP: SEXPTYPE = 19;
pub const EXPRSXP: SEXPTYPE = 20;
pub const BCODESXP: SEXPTYPE = 21;
pub const EXTPTRSXP: SEXPTYPE = 22;
pub const WEAKREFSXP: SEXPTYPE = 23;
pub const RAWSXP: SEXPTYPE = 24;
pub const OBJSXP: SEXPTYPE = 25;

// pre-defined symbols
extern "C" {
    pub static mut R_NamesSymbol: SEXP;
    pub static mut R_ClassSymbol: SEXP;
    pub static mut R_DimSymbol: SEXP;
}

// NULL
extern "C" {
    pub static mut R_NilValue: SEXP;
}

// NA
extern "C" {
    pub static mut R_NaInt: ::std::os::raw::c_int;
    pub static mut R_NaReal: f64;
    pub static mut R_NaString: SEXP;

    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
}

// Allocation and attributes
extern "C" {
    pub fn Rf_xlength(arg1: SEXP) -> R_xlen_t;
    pub fn SETLENGTH(x: SEXP, v: R_xlen_t);
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_install(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
}

// Integer
extern "C" {
    pub fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_isInteger(arg1: SEXP) -> Rboolean;
}

// Real
extern "C" {
    pub fn REAL(x: SEXP) -> *mut f64;
    pub fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64;
    pub fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64);
    pub fn Rf_ScalarReal(arg1: f64) -> SEXP;
    pub fn Rf_isReal(s: SEXP) -> Rboolean;
}

// Complex
//
// Since the representation of Rcomplex matches num_complex's Compplex64, use it
// directly. Note that num-complex's docment warns as following and this seems
// the case of passing as a value.
//
//     Note that `Complex<F>` where `F` is a floating point type is **only**
//     memory layout compatible with C’s complex types, **not** necessarily
//     calling convention compatible. This means that for FFI you can only pass
//     `Complex<F>` behind a pointer, not as a value.
//     (https://docs.rs/num-complex/latest/num_complex/struct.Complex.html#representation-and-foreign-function-interface-compatibility)
//
// While it's true it's not guaranteed to be safe, in actual, no problem has
// benn found so far, and it's a common attitude to ignore the unsafety.
//
// cf. https://gitlab.com/petsc/petsc-rs/-/issues/1
#[cfg(feature = "complex")]
extern "C" {
    pub fn COMPLEX(x: SEXP) -> *mut num_complex::Complex64;
    pub fn COMPLEX_ELT(x: SEXP, i: R_xlen_t) -> num_complex::Complex64;
    pub fn SET_COMPLEX_ELT(x: SEXP, i: R_xlen_t, v: num_complex::Complex64);
    pub fn Rf_ScalarComplex(arg1: num_complex::Complex64) -> SEXP;
    pub fn Rf_isComplex(s: SEXP) -> Rboolean;
}

// Logical
extern "C" {
    pub fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn SET_LOGICAL_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn Rf_ScalarLogical(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_isLogical(s: SEXP) -> Rboolean;
}

// String and character

pub const cetype_t_CE_NATIVE: cetype_t = 0;
pub const cetype_t_CE_UTF8: cetype_t = 1;
pub const cetype_t_CE_LATIN1: cetype_t = 2;
pub const cetype_t_CE_BYTES: cetype_t = 3;
pub const cetype_t_CE_SYMBOL: cetype_t = 5;
pub const cetype_t_CE_ANY: cetype_t = 99;
pub type cetype_t = ::std::os::raw::c_int;

extern "C" {
    pub fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP);
    pub fn Rf_ScalarString(arg1: SEXP) -> SEXP;
    pub fn Rf_isString(s: SEXP) -> Rboolean;

    pub fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_mkCharLenCE(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: cetype_t,
    ) -> SEXP;
}

// List
extern "C" {
    pub fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP;
}

// External pointer

pub type R_CFinalizer_t = ::std::option::Option<unsafe extern "C" fn(arg1: SEXP)>;
extern "C" {
    pub fn R_MakeExternalPtr(p: *mut ::std::os::raw::c_void, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddr(s: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn R_ClearExternalPtr(s: SEXP);

    pub fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean);
}

// Pairlist
extern "C" {
    pub fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn SETCAR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SET_TAG(x: SEXP, y: SEXP);
}

// Function and environment
extern "C" {
    pub fn Rf_isFunction(arg1: SEXP) -> Rboolean;
    pub fn Rf_isEnvironment(arg1: SEXP) -> Rboolean;
    pub fn Rf_eval(arg1: SEXP, arg2: SEXP) -> SEXP;

    pub static mut R_GlobalEnv: SEXP;
}

// Parse
pub const ParseStatus_PARSE_NULL: ParseStatus = 0;
pub const ParseStatus_PARSE_OK: ParseStatus = 1;
pub const ParseStatus_PARSE_INCOMPLETE: ParseStatus = 2;
pub const ParseStatus_PARSE_ERROR: ParseStatus = 3;
pub const ParseStatus_PARSE_EOF: ParseStatus = 4;
pub type ParseStatus = ::std::os::raw::c_int;
extern "C" {
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;

    pub fn R_compute_identical(arg1: SEXP, arg2: SEXP, arg3: ::std::os::raw::c_int) -> Rboolean;
}

// Protection
extern "C" {
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::std::os::raw::c_int);
    pub fn R_PreserveObject(arg1: SEXP);
}

// Type
extern "C" {
    // Note: For some reason, the return type of TYPEOF() is defined as int in
    // RInternals.h and memory.c. However, the actual implementation is
    //
    // In memory.c:
    //
    //     int (TYPEOF)(SEXP x) { return TYPEOF(CHK(x)); }
    //
    // In Defn.h:
    //
    //     #define TYPEOF(x)    ((x)->sxpinfo.type)
    //
    // and the definition of the `type` field of `sxpinfo_struct` is `SEXPTYPE`,
    // so the actual return type should be `SEXPTYPE`, while I'm not 100% sure...
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
    pub fn Rf_type2char(arg1: SEXPTYPE) -> *const ::std::os::raw::c_char;
}

// Error
extern "C" {
    pub fn Rf_errorcall(arg1: SEXP, arg2: *const ::std::os::raw::c_char, ...) -> !;
}

// I/O
extern "C" {
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
}
