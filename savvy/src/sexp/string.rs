use std::ffi::CStr;

use libR_sys::{
    cetype_t_CE_UTF8, Rf_mkCharLenCE, Rf_translateCharUTF8, Rf_xlength, SET_STRING_ELT, SEXP,
    STRING_ELT, STRSXP,
};

use super::na::NotAvailableValue;
use super::Sxp;
use crate::protect;

pub struct StringSxp(pub SEXP);
pub struct OwnedStringSxp {
    inner: SEXP,
    token: SEXP,
    len: usize,
}

impl StringSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> StringSxpIter {
        StringSxpIter {
            sexp: &self.0,
            i: 0,
            len: self.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedStringSxp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> StringSxp {
        StringSxp(self.inner)
    }

    pub fn iter(&self) -> StringSxpIter {
        StringSxpIter {
            sexp: &self.inner,
            i: 0,
            len: self.len,
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: &str) {
        if i >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            );
        }
        unsafe {
            // We might be able to put `R_NaString` directly without using
            // <&str>::na(), but probably this is an inevitable cost of
            // providing <&str>::na().
            let v_sexp = if v.is_na() {
                libR_sys::R_NaString
            } else {
                Rf_mkCharLenCE(v.as_ptr() as *const i8, v.len() as i32, cetype_t_CE_UTF8)
            };

            SET_STRING_ELT(self.inner, i as _, v_sexp);
        }
    }

    pub fn new(len: usize) -> crate::Result<Self> {
        let inner = crate::alloc_vector(STRSXP, len as _)?;
        let token = protect::insert_to_preserved_list(inner);
        Ok(Self { inner, token, len })
    }
}

impl Drop for OwnedStringSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for StringSxp {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        if !value.is_string() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to string");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl<T> From<&[T]> for OwnedStringSxp
where
    T: AsRef<str>, // This works both for &str and String
{
    fn from(value: &[T]) -> Self {
        let mut out = Self::new(value.len()).expect("Couldn't allocate vector");
        value
            .iter()
            .enumerate()
            .for_each(|(i, v)| out.set_elt(i, v.as_ref()));
        out
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<StringSxp> for SEXP {
    fn from(value: StringSxp) -> Self {
        value.inner()
    }
}

impl From<OwnedStringSxp> for SEXP {
    fn from(value: OwnedStringSxp) -> Self {
        value.inner()
    }
}

pub struct StringSxpIter<'a> {
    pub sexp: &'a SEXP,
    i: usize,
    len: usize,
}

impl<'a> Iterator for StringSxpIter<'a> {
    // The lifetime here is 'static, not 'a, in the assumption that
    // `Rf_translateCharUTF8()` allocate the string on R's side so it should be
    // there until the R session ends.
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        unsafe {
            let e = STRING_ELT(*self.sexp, i as _);

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
