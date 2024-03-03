#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]

// internal types

use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

pub type R_xlen_t = isize;

pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_int;

// SEXP
#[derive(Clone)]
pub enum SexpData {
    Integer(Vec<i32>),
    Real(Vec<f64>),
    Logical(Vec<i32>),
    String(Vec<String>),
    Char(String),
    List(Vec<SEXP>),
    Symbol(&'static str),
    Null,
}

#[derive(Clone)]
pub struct SexpMock {
    data: SexpData,
    attrib: HashMap<String, SEXP>,
}

impl SexpMock {
    fn new(data: SexpData) -> Self {
        Self {
            data,
            attrib: HashMap::new(),
        }
    }
}

pub type SEXP = *mut SexpMock;

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

// pre-defined symbols
// This is a bit tricky. Rust doesn't allow to create a static *mut SexpMock.
pub static mut R_NamesSymbol: SEXP = "names".as_ptr() as _;
pub static mut R_ClassSymbol: SEXP = "class".as_ptr() as _;
pub static mut R_DimSymbol: SEXP = "dim".as_ptr() as _;

// NULL
pub static mut R_NilValue: SEXP = std::ptr::null_mut() as _;

// NA
pub static mut R_NaInt: ::std::os::raw::c_int = ::std::os::raw::c_int::MIN;

// https://github.com/wch/r-source/blob/b4b3a905e862489ad9d70ab8580b3b453c24bbe5/src/main/arithmetic.c#L90-L98
const fn R_ValueOfNA() -> f64 {
    // 000007a2 = 1954
    let b = 0x7ff00000_000007a2_i64.to_be_bytes();
    let u = u64::from_be_bytes(b);
    unsafe { std::mem::transmute::<u64, f64>(u) }
}
pub static mut R_NaReal: f64 = R_ValueOfNA();

pub static mut R_NaString: SEXP = "".as_ptr() as _;

pub unsafe fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int {
    if arg1 == unsafe { R_NaReal } {
        1
    } else {
        2
    }
}

// Allocation and attributes
/// # Safety
/// This is for testing only
pub unsafe fn Rf_xlength(arg1: SEXP) -> R_xlen_t {
    match &(*arg1).data {
        SexpData::Integer(v) => v.len() as R_xlen_t,
        SexpData::Real(v) => v.len() as R_xlen_t,
        SexpData::Logical(v) => v.len() as R_xlen_t,
        SexpData::String(v) => v.len() as R_xlen_t,
        SexpData::Char(_) => 1,
        SexpData::List(v) => v.len() as R_xlen_t,
        SexpData::Symbol(_) => 1,
        SexpData::Null => 0,
    }
}

pub unsafe fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP {
    let out = match arg1 {
        INTSXP => SexpMock::new(SexpData::Integer(vec![0_i32; arg2 as usize])),
        REALSXP => SexpMock::new(SexpData::Real(vec![0_f64; arg2 as usize])),
        LGLSXP => SexpMock::new(SexpData::Logical(vec![0_i32; arg2 as usize])),
        STRSXP => SexpMock::new(SexpData::String(vec!["".to_string(); arg2 as usize])),
        VECSXP => SexpMock::new(SexpData::List(vec![R_NilValue; arg2 as usize])),
        _ => return R_NilValue,
    };

    Box::into_raw(Box::new(out))
}

pub unsafe fn Rf_install(arg1: *const ::std::os::raw::c_char) -> SEXP {
    let c_str = CStr::from_ptr(arg1).to_string_lossy();
    let out = SexpMock::new(SexpData::Symbol(c_str.to_string().leak()));
    Box::into_raw(Box::new(out))
}

unsafe fn get_key(arg2: *mut SexpMock) -> Option<String> {
    let key = if arg2 == R_NamesSymbol {
        "names"
    } else if arg2 == R_ClassSymbol {
        "class"
    } else if arg2 == R_DimSymbol {
        "dim"
    } else {
        match (*arg2).data {
            SexpData::Symbol(s) => s,
            _ => return None,
        }
    };

    Some(key.to_string())
}

pub unsafe fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP {
    let key = match get_key(arg2) {
        Some(value) => value,
        None => return R_NilValue,
    };

    *(*arg1).attrib.get(&key).unwrap_or(&R_NilValue)
}

pub unsafe fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP {
    let key = match get_key(arg2) {
        Some(value) => value,
        None => return R_NilValue,
    };

    (*arg1).attrib.insert(key, arg3);

    // I don't know what value Rf_setAttrib() returns, but this should be fine
    R_NilValue
}

// Integer
pub unsafe fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int {
    match &mut (*x).data {
        SexpData::Integer(v) => v.as_mut_ptr(),
        _ => panic!("A non-integer SEXP is passed to INTEGER()"),
    }
}
pub unsafe fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int {
    match &(*x).data {
        SexpData::Integer(data) => data[i as usize],
        _ => panic!("A non-integer SEXP is passed to INTEGER_ELT()"),
    }
}
pub unsafe fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int) {
    match &mut (*x).data {
        SexpData::Integer(data) => {
            data[i as usize] = v;
        }
        _ => panic!("A non-integer SEXP is passed to SET_INTEGER_ELT()"),
    }
}
pub unsafe fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP {
    let out = Rf_allocVector(INTSXP, 1);
    SET_INTEGER_ELT(out, 0, arg1);
    out
}
pub unsafe fn Rf_isInteger(arg1: SEXP) -> Rboolean {
    match (*arg1).data {
        SexpData::Integer(_) => 1,
        _ => 0,
    }
}

// Real
pub unsafe fn REAL(x: SEXP) -> *mut f64 {
    match &mut (*x).data {
        SexpData::Real(v) => v.as_mut_ptr(),
        _ => panic!("A non-real SEXP is passed to REAL()"),
    }
}
pub unsafe fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64 {
    match &(*x).data {
        SexpData::Real(data) => data[i as usize],
        _ => panic!("A non-real SEXP is passed to REAL_ELT()"),
    }
}
pub unsafe fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64) {
    match &mut (*x).data {
        SexpData::Real(data) => {
            data[i as usize] = v;
        }
        _ => panic!("A non-real SEXP is passed to SET_REAL_ELT()"),
    }
}
pub unsafe fn Rf_ScalarReal(arg1: f64) -> SEXP {
    let out = Rf_allocVector(REALSXP, 1);
    SET_REAL_ELT(out, 0, arg1);
    out
}
pub unsafe fn Rf_isReal(s: SEXP) -> Rboolean {
    match (*s).data {
        SexpData::Real(_) => 1,
        _ => 0,
    }
}

// Logical
pub unsafe fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int {
    match &mut (*x).data {
        SexpData::Logical(v) => v.as_mut_ptr(),
        _ => panic!("A non-logical SEXP is passed to LOGICAL()"),
    }
}
pub unsafe fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int {
    match &(*x).data {
        SexpData::Logical(data) => data[i as usize],
        _ => panic!("A non-logical SEXP is passed to LOGICAL_ELT()"),
    }
}
pub unsafe fn SET_LOGICAL_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int) {
    match &mut (*x).data {
        SexpData::Logical(data) => {
            data[i as usize] = v;
        }
        _ => panic!("A non-logical SEXP is passed to SET_LOGICAL_ELT()"),
    }
}
pub unsafe fn Rf_ScalarLogical(arg1: ::std::os::raw::c_int) -> SEXP {
    let out = Rf_allocVector(LGLSXP, 1);
    SET_LOGICAL_ELT(out, 0, arg1);
    out
}
pub unsafe fn Rf_isLogical(s: SEXP) -> Rboolean {
    match (*s).data {
        SexpData::Logical(_) => 1,
        _ => 0,
    }
}

// String and character

pub const cetype_t_CE_NATIVE: cetype_t = 0;
pub const cetype_t_CE_UTF8: cetype_t = 1;
pub const cetype_t_CE_LATIN1: cetype_t = 2;
pub const cetype_t_CE_BYTES: cetype_t = 3;
pub const cetype_t_CE_SYMBOL: cetype_t = 5;
pub const cetype_t_CE_ANY: cetype_t = 99;
pub type cetype_t = ::std::os::raw::c_int;

pub unsafe fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP {
    match &(*x).data {
        SexpData::String(data) => {
            let s = data[i as usize].clone();
            let out = SexpMock::new(SexpData::Char(s));

            Box::into_raw(Box::new(out))
        }
        _ => panic!("A non-string SEXP is passed to STRING_ELT()"),
    }
}
pub unsafe fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP) {
    let new_value = match &mut (*v).data {
        SexpData::Char(data) => data.clone(),
        _ => panic!("A non-character SEXP is passed to SET_STRING_ELT()"),
    };

    match &mut (*x).data {
        SexpData::String(data) => {
            data[i as usize] = new_value;
        }
        _ => panic!("A non-string SEXP is passed to SET_STRING_ELT()"),
    }
}
pub unsafe fn Rf_ScalarString(arg1: SEXP) -> SEXP {
    match &(*arg1).data {
        SexpData::Char(data) => {
            let out = Rf_allocVector(STRSXP, 1);
            SET_STRING_ELT(out, 0, arg1);
            out
        }
        _ => panic!("A non-character SEXP is passed to Rf_ScalarString()"),
    }
}
pub unsafe fn Rf_isString(s: SEXP) -> Rboolean {
    match (*s).data {
        SexpData::String(_) => 1,
        _ => 0,
    }
}

pub unsafe fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char {
    match &(*x).data {
        SexpData::Char(data) => CString::new(data.as_str()).unwrap().into_raw(),
        _ => panic!("A non-character SEXP is passed to R_CHAR()"),
    }
}
pub unsafe fn Rf_mkCharLenCE(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    arg3: cetype_t,
) -> SEXP {
    let c_str = CStr::from_ptr(arg1).to_string_lossy();
    let out = SexpMock::new(SexpData::Char(c_str.to_string()));
    Box::into_raw(Box::new(out))
}

// List
pub unsafe fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP {
    match &(*x).data {
        SexpData::List(data) => data[i as usize],
        _ => panic!("A non-list SEXP is passed to VECTOR_ELT()"),
    }
}
pub unsafe fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP {
    match &mut (*x).data {
        SexpData::List(data) => {
            data[i as usize] = v;

            // I don't know what value SET_VECTOR_ELT() returns, but this should be fine
            R_NilValue
        }
        _ => panic!("A non-list SEXP is passed to SET_VECTOR_ELT()"),
    }
}

// External pointer

pub type R_CFinalizer_t = ::std::option::Option<unsafe extern "C" fn(arg1: SEXP)>;
pub unsafe fn R_MakeExternalPtr(p: *mut ::std::os::raw::c_void, tag: SEXP, prot: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn R_ExternalPtrAddr(s: SEXP) -> *mut ::std::os::raw::c_void {
    unimplemented!();
}
pub unsafe fn R_ClearExternalPtr(s: SEXP) {
    unimplemented!();
}
pub unsafe fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean) {
    unimplemented!();
}

// Pairlist
pub unsafe fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn CAR(e: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn CDR(e: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn SETCAR(x: SEXP, y: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn SETCDR(x: SEXP, y: SEXP) -> SEXP {
    unimplemented!();
}
pub unsafe fn SET_TAG(x: SEXP, y: SEXP) {
    unimplemented!();
}

// Function and environment
pub unsafe fn Rf_isFunction(arg1: SEXP) -> Rboolean {
    unimplemented!();
}
pub unsafe fn Rf_isEnvironment(arg1: SEXP) -> Rboolean {
    unimplemented!();
}
pub unsafe fn Rf_eval(arg1: SEXP, arg2: SEXP) -> SEXP {
    unimplemented!();
}

pub static mut R_GlobalEnv: SEXP = std::ptr::null_mut() as _;

// protection
pub unsafe fn Rf_protect(arg1: SEXP) -> SEXP {
    // Do nothing
    arg1
}
pub unsafe fn Rf_unprotect(arg1: ::std::os::raw::c_int) {
    // Do nothing
}
pub unsafe fn R_PreserveObject(arg1: SEXP) {
    // Do nothing
}

// type
pub unsafe fn TYPEOF(x: SEXP) -> ::std::os::raw::c_int {
    match &(*x).data {
        SexpData::Integer(_) => INTSXP as _,
        SexpData::Real(_) => REALSXP as _,
        SexpData::Logical(_) => LGLSXP as _,
        SexpData::String(_) => STRSXP as _,
        SexpData::Char(_) => CHARSXP as _,
        SexpData::List(_) => VECSXP as _,
        SexpData::Symbol(_) => SYMSXP as _,
        SexpData::Null => NILSXP as _,
    }
}
pub unsafe fn Rf_type2char(arg1: SEXPTYPE) -> *const ::std::os::raw::c_char {
    let type_char = match arg1 {
        INTSXP => "integer",
        REALSXP => "real",
        LGLSXP => "logical",
        STRSXP => "string",
        CHARSXP => "character",
        VECSXP => "list",
        SYMSXP => "symbol",
        NILSXP => "null",
        _ => "(unsupported type in the fake-libR)",
    };

    CString::new(type_char).unwrap().into_raw()
}

// error
pub unsafe fn Rf_errorcall(arg1: SEXP, arg2: *const ::std::os::raw::c_char) -> ! {
    unimplemented!();
}

// I/O
pub unsafe fn Rprintf(arg1: *const ::std::os::raw::c_char) {
    // Do nothing
}
pub unsafe fn REprintf(arg1: *const ::std::os::raw::c_char) {
    // Do nothing
}
