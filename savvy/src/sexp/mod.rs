use std::ffi::CStr;

use savvy_ffi::{
    Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString, Rf_type2char, EXTPTRSXP, SEXP, TYPEOF,
    VECSXP,
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
        unsafe {
            // TODO: replace this `R_typeToChar()` which will be introduced in R 4.4
            let c = Rf_type2char(TYPEOF(self.0) as _);
            CStr::from_ptr(c).to_str().unwrap()
        }
    }
}
