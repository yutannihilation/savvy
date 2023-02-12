use std::{ffi::CStr, ops::Index};

use anyhow::{anyhow, Error, Result};
use libR_sys::{
    Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString, Rf_translateCharUTF8, Rf_xlength,
    INTEGER_ELT, SEXP, STRING_ELT,
};

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

    fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.0) != 0 }
    }

    fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.0) != 0 }
    }

    fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.0) != 0 }
    }
}

pub struct IntegerSxp(SEXP);

impl IntegerSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn elt(&self, i: usize) -> i32 {
        unsafe { INTEGER_ELT(self.0, i as _) }
    }
}

impl TryFrom<SEXP> for IntegerSxp {
    type Error = Error;

    fn try_from(value: SEXP) -> Result<Self> {
        if !Sxp(value).is_integer() {
            return Err(error::UnextendrError::UnexpectedType("???".to_string()).into());
        }
        Ok(Self(value))
    }
}

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for IntegerSxp {
//         type Output = i32;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct StringSxp(SEXP);

impl StringSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> SEXP {
        unsafe { STRING_ELT(self.0, i as _) }
    }

    pub fn iter(&self) -> StringSxpIter {
        StringSxpIter {
            sexp: &self,
            i: 0,
            len: self.len(),
        }
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

pub struct StringSxpIter<'a> {
    pub sexp: &'a StringSxp,
    i: usize,
    len: usize,
}

impl<'a> Iterator for StringSxpIter<'a> {
    // There lifetime here is 'static, not 'a, in the assumption that
    // `Rf_translateCharUTF8()` will allocate it on R's side so it should be
    // there until the R session ends.
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        unsafe {
            let e = self.sexp.elt(i);

            // Because `None` means the end of the iterator, we cannot return
            // `None` even for missing values. So, if we want to reject missing
            // values, we might need some extra mechanism.
            if e == libR_sys::R_NaString {
                return Some("");
            }

            // NOTE: after this point, we no longer can know if the element was
            // a missing value. extendr tries to propagate the missingnaess by
            // introducing a sentinel value, but it looks broken to my eyes.
            //
            // - https://github.com/extendr/extendr/blob/60f232f0379777cc864de0851d456706456d1845/extendr-api/src/iter.rs#L65-L66
            // - https://github.com/extendr/extendr/pull/477#issuecomment-1423452814
            let e_utf8 = Rf_translateCharUTF8(e);

            // As `e_utf8` is translated into UTF-8, it must be a valid UTF-8
            // data, so we just unwrap it.
            Some(CStr::from_ptr(e_utf8).to_str().unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for StringSxpIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}
