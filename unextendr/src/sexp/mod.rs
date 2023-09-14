use libR_sys::{
    Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString, EXTPTRSXP, SEXP, TYPEOF, VECSXP,
};

pub mod external_pointer;
pub mod integer;
pub mod list;
pub mod logical;
pub mod na;
pub mod null;
pub mod real;
pub mod scalar;
pub mod string;

pub struct Sxp(pub SEXP);

impl Sxp {
    // There are two versions of Rf_isString(), but anyway this should be cheap.
    //
    // macro version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/include/Defn.h#L759
    // function version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/main/memory.c#L4460
    pub fn is_string(&self) -> bool {
        unsafe { Rf_isString(self.0) == 1 }
    }

    pub fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.0) == 1 }
    }

    pub fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.0) == 1 }
    }

    pub fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.0) == 1 }
    }

    // There's no test function for VECSXP. Rf_isList() is for pairlist
    pub fn is_list(&self) -> bool {
        unsafe { TYPEOF(self.0) as u32 == VECSXP }
    }

    pub fn is_external_pointer(&self) -> bool {
        unsafe { TYPEOF(self.0) as u32 == EXTPTRSXP }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_human_readable_type_name(&self) -> &'static str {
        match unsafe { TYPEOF(self.0) as u32 } {
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
}
