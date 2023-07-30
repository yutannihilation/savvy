use std::ffi::CStr;

use libR_sys::{
    cetype_t_CE_UTF8, Rf_allocVector, Rf_mkCharLenCE, Rf_translateCharUTF8, Rf_xlength,
    SET_STRING_ELT, SEXP, STRING_ELT, STRSXP,
};

use super::na::NotAvailableValue;
use super::Sxp;
use crate::{error::get_human_readable_type_name, protect};

pub struct StringSxp(SEXP);
pub struct OwnedStringSxp {
    inner: StringSxp,
    token: SEXP,
}

impl StringSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> SEXP {
        unsafe { STRING_ELT(self.0, i as _) }
    }

    pub fn iter(&self) -> StringSxpIter {
        StringSxpIter {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedStringSxp {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn elt(&self, i: usize) -> SEXP {
        self.inner.elt(i)
    }

    pub fn iter(&self) -> StringSxpIter {
        self.inner.iter()
    }

    pub(crate) fn inner(&self) -> SEXP {
        self.inner.inner()
    }

    pub fn set_elt(&mut self, i: usize, v: &str) {
        unsafe {
            // We might be able to put `R_NaString` directly without using
            // <&str>::na(), but probably this is an inevitable cost of
            // providing <&str>::na().
            let v_sexp = if v.is_na() {
                libR_sys::R_NaString
            } else {
                Rf_mkCharLenCE(v.as_ptr() as *const i8, v.len() as i32, cetype_t_CE_UTF8)
            };

            SET_STRING_ELT(self.inner(), i as _, v_sexp);
        }
    }

    pub fn new(len: usize) -> Self {
        let out = unsafe { Rf_allocVector(STRSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);
        Self {
            inner: StringSxp(out),
            token,
        }
    }
}

impl Drop for OwnedStringSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<SEXP> for StringSxp {
    type Error = crate::error::Error;

    fn try_from(value: SEXP) -> crate::error::Result<Self> {
        if !Sxp(value).is_string() {
            let type_name = get_human_readable_type_name(value);
            let msg = format!("Cannot convert {type_name} to string");
            return Err(crate::error::Error::UnexpectedType(msg));
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
            // `None` even for missing values.
            if e == libR_sys::R_NaString {
                return Some(Self::Item::na());
            }

            let e_utf8 = Rf_translateCharUTF8(e);

            // As `e_utf8` is translated into UTF-8, it must be a valid UTF-8
            // data, so we just unwrap it without any aditional check.
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
