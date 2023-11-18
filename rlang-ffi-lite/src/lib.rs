// internal types

pub type R_xlen_t = isize;

pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_int;

// SEXP
pub type SEXP = *mut ::std::os::raw::c_void;

// SEXPTYPE

pub type SEXPTYPE = ::std::os::raw::c_uint;

pub const NILSXP: u32 = 0;
pub const SYMSXP: u32 = 1;
pub const LISTSXP: u32 = 2;
pub const CLOSXP: u32 = 3;
pub const ENVSXP: u32 = 4;
pub const PROMSXP: u32 = 5;
pub const LANGSXP: u32 = 6;
pub const SPECIALSXP: u32 = 7;
pub const BUILTINSXP: u32 = 8;
pub const CHARSXP: u32 = 9;
pub const LGLSXP: u32 = 10;
pub const INTSXP: u32 = 13;
pub const REALSXP: u32 = 14;
pub const CPLXSXP: u32 = 15;
pub const STRSXP: u32 = 16;
pub const DOTSXP: u32 = 17;
pub const ANYSXP: u32 = 18;
pub const VECSXP: u32 = 19;
pub const EXPRSXP: u32 = 20;
pub const BCODESXP: u32 = 21;
pub const EXTPTRSXP: u32 = 22;
pub const WEAKREFSXP: u32 = 23;
pub const RAWSXP: u32 = 24;
pub const OBJSXP: u32 = 25;

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
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
}

// Integer
extern "C" {
    pub fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP;
}

// Real
extern "C" {
    pub fn REAL(x: SEXP) -> *mut f64;
    pub fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64;
    pub fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64);
    pub fn Rf_ScalarReal(arg1: f64) -> SEXP;
}

// Logical
extern "C" {
    pub fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn SET_LOGICAL_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn Rf_ScalarLogical(arg1: ::std::os::raw::c_int) -> SEXP;
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
    pub fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_mkCharLenCE(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: cetype_t,
    ) -> SEXP;
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
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn SETCAR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SET_TAG(x: SEXP, y: SEXP);
}

// protection
extern "C" {
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::std::os::raw::c_int);
    pub fn R_PreserveObject(arg1: SEXP);
}

// error
extern "C" {
    pub fn Rf_errorcall(arg1: SEXP, arg2: *const ::std::os::raw::c_char, ...) -> !;
}

// I/O
extern "C" {
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
}
