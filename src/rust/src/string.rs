use std::ffi::CStr;

use libR_sys::{Rf_translateCharUTF8, Rf_xlength, SEXP, STRING_ELT};

use crate::sexp::Sxp;

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
    type Error = anyhow::Error;

    fn try_from(value: SEXP) -> anyhow::Result<Self> {
        if !Sxp(value).is_string() {
            return Err(crate::error::UnextendrError::UnexpectedType("???".to_string()).into());
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
