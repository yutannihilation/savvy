use libR_sys::{SEXP, TYPEOF};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnextendrError {
    #[error("Unexpected type: {0}")]
    UnexpectedType(String),
    #[error("Unknown error happened")]
    Unknown,
}

pub fn get_human_readable_type_name(x: SEXP) -> &'static str {
    match unsafe { TYPEOF(x) as u32 } {
        libR_sys::NILSXP => "NULL",
        libR_sys::SYMSXP => "a symbol",
        libR_sys::LISTSXP => "a pairlist",
        libR_sys::CLOSXP => "a closure",
        libR_sys::ENVSXP => "a environment",
        libR_sys::PROMSXP => "a promise",
        libR_sys::LANGSXP => "a language",
        libR_sys::SPECIALSXP => "a special function",
        libR_sys::BUILTINSXP => "a builtin function",
        libR_sys::CHARSXP => "string",
        libR_sys::LGLSXP => "logical",
        libR_sys::INTSXP => "integer",
        libR_sys::REALSXP => "real",
        libR_sys::CPLXSXP => "complex",
        libR_sys::STRSXP => "string",
        libR_sys::DOTSXP => "dot",
        libR_sys::ANYSXP => "ANYSXP",
        libR_sys::VECSXP => "a list",
        libR_sys::EXPRSXP => "expression",
        libR_sys::BCODESXP => "byte code",
        libR_sys::EXTPTRSXP => "external pointer",
        libR_sys::WEAKREFSXP => "weak reference",
        libR_sys::RAWSXP => "raw vector",
        libR_sys::S4SXP => "S4 object",
        _ => "Unknown",
    }
}
