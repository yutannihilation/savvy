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

    pub fn set_elt(&mut self, i: usize, v: &str) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }
        unsafe {
            SET_STRING_ELT(self.inner, i as _, str_to_charsxp(v)?);
        }

        Ok(())
    }

    pub fn new(len: usize) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(STRSXP, len as _)?;
        Self::new_from_raw_sexp(inner, len)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);
        Ok(Self { inner, token, len })
    }
}

unsafe fn str_to_charsxp(v: &str) -> crate::error::Result<SEXP> {
    // We might be able to put `R_NaString` directly without using
    // <&str>::na(), but probably this is an inevitable cost of
    // providing <&str>::na().
    if v.is_na() {
        Ok(libR_sys::R_NaString)
    } else {
        crate::unwind_protect(|| {
            Rf_mkCharLenCE(v.as_ptr() as *const i8, v.len() as i32, cetype_t_CE_UTF8)
        })
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

impl<T> TryFrom<&[T]> for OwnedStringSxp
where
    T: AsRef<str>, // This works both for &str and String
{
    type Error = crate::error::Error;

    fn try_from(value: &[T]) -> crate::error::Result<Self> {
        let mut out = Self::new(value.len())?;
        for (i, v) in value.iter().enumerate() {
            out.set_elt(i, v.as_ref())?;
        }
        Ok(out)
    }
}

impl TryFrom<&str> for OwnedStringSxp {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> crate::error::Result<Self> {
        let sexp = unsafe {
            // Note: unlike `new()`, this allocates a STRSXP after creating a
            // CHARSXP. So, the `CHARSXP` needs to be protected.
            let charsxp = str_to_charsxp(value)?;
            libR_sys::Rf_protect(charsxp);
            let out = crate::unwind_protect(|| libR_sys::Rf_ScalarString(charsxp))?;
            libR_sys::Rf_unprotect(1);
            out
        };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

// TODO: if I turn this to `impl<T: AsRef<str>> TryFrom<T>`, the compiler warns
// this is a conflicting implementation. Why...?
impl TryFrom<String> for OwnedStringSxp {
    type Error = crate::error::Error;

    fn try_from(value: String) -> crate::error::Result<Self> {
        OwnedStringSxp::try_from(value.as_str())
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
