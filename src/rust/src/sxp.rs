use std::{ffi::CStr, ops::Index};

use anyhow::{anyhow, Error, Result};
use libR_sys::{Rf_isString, Rf_translateCharUTF8, Rf_xlength, SEXP, STRING_ELT};

use crate::error;

pub struct Sxp(SEXP);

impl Sxp {
    // There are two versions of Rf_isString(), but anyway this should be cheap.
    //
    // macro version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/include/Defn.h#L759
    // function version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/main/memory.c#L4460
    fn is_string(&self) -> bool {
        unsafe { Rf_isString(self.0) != 0 }
    }
}

pub struct StringSxp(SEXP);

impl StringSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn elt(&self, i: usize) -> SEXP {
        unsafe { STRING_ELT(self.0, i as _) }
        // if e == libR_sys::R_NaString {
        //     return Err();
        // }
        // let e_cstr = CStr::from_ptr(Rf_translateCharUTF8(e));
        // r_eprint(format!("{:?}\n", e_cstr.to_str()));
    }
}

impl TryFrom<SEXP> for StringSxp {
    type Error = Error;

    fn try_from(value: SEXP) -> Result<Self> {
        if !Sxp(value).is_string() {
            return Err(error::UnextendrError::UnexpectedType("???".to_string()).into());
        }
        Ok(Self(value))
    }
}

impl Index<usize> for StringSxp {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        let e = self.elt(index);
        unsafe {
            // NOTE: after this point, we no longer can know if the element was
            // a missing value. extendr tries to propagate the missingnaess by
            // introducing a sentinel value, but it looks broken to my eyes.
            //
            // - https://github.com/extendr/extendr/blob/60f232f0379777cc864de0851d456706456d1845/extendr-api/src/iter.rs#L65-L66
            // - https://github.com/extendr/extendr/pull/477#issuecomment-1423452814
            let e_utf8 = Rf_translateCharUTF8(e);

            // As `e_utf8` is translated into UTF-8, it should be a valid UTF-8
            // data, so we just unwrap() without considering the invalid cases.
            CStr::from_ptr(e_utf8).to_str().unwrap()
        }
    }
}
