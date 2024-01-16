use std::ffi::CStr;

use savvy_ffi::{
    cetype_t_CE_UTF8, Rf_mkCharLenCE, Rf_xlength, R_CHAR, SET_STRING_ELT, SEXP, STRING_ELT, STRSXP,
};

use super::na::NotAvailableValue;
use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect;

/// An external SEXP of a character vector.
pub struct StringSexp(pub SEXP);

/// A newly-created SEXP of a character vector.
pub struct OwnedStringSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(StringSexp);
impl_common_sexp_ops_owned!(OwnedStringSexp);

impl StringSexp {
    pub fn iter(&self) -> StringSexpIter {
        StringSexpIter {
            sexp: &self.0,
            i: 0,
            len: self.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        self.iter().collect()
    }
}

impl OwnedStringSexp {
    pub fn as_read_only(&self) -> StringSexp {
        StringSexp(self.inner)
    }

    pub fn iter(&self) -> StringSexpIter {
        StringSexpIter {
            sexp: &self.inner,
            i: 0,
            len: self.len,
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        self.iter().collect()
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

    /// Constructs a new string vector.
    pub fn new(len: usize) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(STRSXP, len as _)?;
        Self::new_from_raw_sexp(inner, len)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);

        // Note: `R_allocVector()` initializes character vectors, so we don't
        // need to do it by ourselves. R-exts (5.9.2 Allocating storage) says:
        //
        // >  One distinction is that whereas the R functions always initialize
        // >  the elements of the vector, allocVector only does so for lists,
        // >  expressions and character vectors (the cases where the elements
        // >  are themselves R objects).

        Ok(Self { inner, token, len })
    }
}

unsafe fn str_to_charsxp(v: &str) -> crate::error::Result<SEXP> {
    // We might be able to put `R_NaString` directly without using
    // <&str>::na(), but probably this is an inevitable cost of
    // providing <&str>::na().
    if v.is_na() {
        Ok(savvy_ffi::R_NaString)
    } else {
        crate::unwind_protect(|| {
            Rf_mkCharLenCE(v.as_ptr() as *const i8, v.len() as i32, cetype_t_CE_UTF8)
        })
    }
}

impl Drop for OwnedStringSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to StringSexp ***************

impl TryFrom<Sexp> for StringSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_string() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected strings, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<StringSexp> for Sexp {
    fn from(value: StringSexp) -> Self {
        Self(value.inner())
    }
}

impl From<StringSexp> for crate::error::Result<Sexp> {
    fn from(value: StringSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to StringSexp ***************

impl<T> TryFrom<&[T]> for OwnedStringSexp
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

impl<T> TryFrom<Vec<T>> for OwnedStringSexp
where
    T: AsRef<str>, // This works both for &str and String
{
    type Error = crate::error::Error;

    fn try_from(value: Vec<T>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<&str> for OwnedStringSexp {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> crate::error::Result<Self> {
        let sexp = unsafe {
            // Note: unlike `new()`, this allocates a STRSXP after creating a
            // CHARSXP. So, the `CHARSXP` needs to be protected.
            let charsxp = str_to_charsxp(value)?;
            savvy_ffi::Rf_protect(charsxp);
            let out = crate::unwind_protect(|| savvy_ffi::Rf_ScalarString(charsxp))?;
            savvy_ffi::Rf_unprotect(1);
            out
        };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

// TODO: if I turn this to `impl<T: AsRef<str>> TryFrom<T>`, the compiler warns
// this is a conflicting implementation. Why...?
impl TryFrom<String> for OwnedStringSexp {
    type Error = crate::error::Error;

    fn try_from(value: String) -> crate::error::Result<Self> {
        OwnedStringSexp::try_from(value.as_str())
    }
}

impl From<OwnedStringSexp> for Sexp {
    fn from(value: OwnedStringSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedStringSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedStringSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

macro_rules! impl_try_from_rust_strings {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedStringSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_strings!(&[&str]);
impl_try_from_rust_strings!(&[String]);
impl_try_from_rust_strings!(Vec<&str>);
impl_try_from_rust_strings!(Vec<String>);
impl_try_from_rust_strings!(&str);
impl_try_from_rust_strings!(String);

// Iterator for StringSexp ***************

pub struct StringSexpIter<'a> {
    pub sexp: &'a SEXP,
    i: usize,
    len: usize,
}

impl<'a> Iterator for StringSexpIter<'a> {
    // The lifetime here is 'static, not 'a, in the assumption that strings in
    // `R_StringHash`, the global `CHARSXP` cache, won't be deleted during the R
    // session.
    //
    // Note that, in order to stick with 'static lifetime, I can't use
    // `Rf_translateCharUTF8()` here because it doesn't use `R_StringHash` and
    // allocates the string on R's side, which means it's not guaranteed to stay
    // during the whole R session.
    //
    // cf.)
    // - https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#The-CHARSXP-cache
    // - https://github.com/wch/r-source/blob/023ada039c86bf9b65983a71110c586b5994e18d/src/main/sysutils.c#L1284-L1296
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
            if e == savvy_ffi::R_NaString {
                return Some(Self::Item::na());
            }

            // I bravely assume all strings are valid UTF-8 and don't use
            // `Rf_translateCharUTF8()`!
            let ptr = R_CHAR(e) as *const u8;
            let e_utf8 = std::slice::from_raw_parts(ptr, Rf_xlength(e) as usize + 1); // +1 for NUL

            // Use CStr to check the UTF-8 validity.
            Some(
                CStr::from_bytes_with_nul_unchecked(e_utf8)
                    .to_str()
                    .unwrap_or_default(),
            )
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for StringSexpIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}
