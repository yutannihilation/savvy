use libR_sys::{Rf_isInteger, Rf_isList, Rf_isLogical, Rf_isReal, Rf_isString, SEXP};

pub mod integer;
pub mod list;
pub mod logical;
pub mod na;
pub mod null;
pub mod real;
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

    pub fn is_list(&self) -> bool {
        unsafe { Rf_isList(self.0) == 1 }
    }
}
