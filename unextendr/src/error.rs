use libR_sys::{SEXP, TYPEOF};

#[derive(Debug)]
pub enum Error {
    UnexpectedType(String),
    Aborted(SEXP),
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedType(type_name) => write!(f, "Unexpected type: {}", type_name),
            Error::Aborted(_) => write!(f, "Aborted due to some error"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_human_readable_type_name(x: SEXP) -> &'static str {
    match unsafe { TYPEOF(x) as u32 } {
        libR_sys::INTSXP => "integer",
        libR_sys::REALSXP => "real",
        libR_sys::STRSXP => "string",
        libR_sys::LGLSXP => "logical",
        libR_sys::VECSXP => "a list",
        libR_sys::NILSXP => "NULL",
        libR_sys::SYMSXP => "a symbol",
        libR_sys::CLOSXP => "a closure",
        libR_sys::ENVSXP => "a environment",
        libR_sys::PROMSXP => "a promise",
        libR_sys::LANGSXP => "a language",
        libR_sys::LISTSXP => "a pairlist",
        libR_sys::SPECIALSXP => "a special function",
        libR_sys::BUILTINSXP => "a builtin function",
        libR_sys::CHARSXP => "string",
        libR_sys::CPLXSXP => "complex",
        libR_sys::DOTSXP => "dot",
        libR_sys::ANYSXP => "ANYSXP",
        libR_sys::EXPRSXP => "expression",
        libR_sys::BCODESXP => "byte code",
        libR_sys::EXTPTRSXP => "external pointer",
        libR_sys::WEAKREFSXP => "weak reference",
        libR_sys::RAWSXP => "raw vector",
        libR_sys::S4SXP => "S4 object",
        _ => "Unknown",
    }
}
