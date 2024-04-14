use std::ffi::CStr;

use savvy_ffi::{R_NaString, Rf_xlength, R_CHAR, SET_STRING_ELT, SEXP, STRING_ELT, STRSXP};

use super::na::NotAvailableValue;
use super::utils::{assert_len, str_to_charsxp};
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

    /// Set the value of the `i`-th element.
    pub fn set_elt(&mut self, i: usize, v: &str) -> crate::error::Result<()> {
        assert_len(self.len, i)?;
        unsafe { self.set_elt_unchecked(i, str_to_charsxp(v)?) };

        Ok(())
    }

    // Set the value of the `i`-th element.
    // Safety: the user has to assure bounds are checked.
    #[inline]
    pub(crate) unsafe fn set_elt_unchecked(&mut self, i: usize, v: SEXP) {
        unsafe { SET_STRING_ELT(self.inner, i as _, v) };
    }

    /// Set the `i`-th element to NA.
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, R_NaString) };

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

    /// Constructs a new real vector from an iterator.
    ///
    /// Note that, if you already have a slice or vec, you can also use
    /// [`try_from_slice`].
    pub fn try_from_iter<I, U>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = U>,
        U: AsRef<str>,
    {
        let iter = iter.into_iter();

        match iter.size_hint() {
            (_, Some(upper)) => {
                // If the maximum length is known, use it at frist. But, the
                // iterator's length might be shorter than the reported one
                // (e.g. `(0..10).filter(|x| x % 2 == 0)`), so it needs to be
                // truncated to the actual length at last.

                // string doesn't have new_without_init() method
                let mut out = Self::new(upper)?;

                let mut last_index = 0;
                for (i, v) in iter.enumerate() {
                    // The upper bound of size_hint() is just for optimization
                    // and what we should not trust. So, we should't use
                    // `set_elt_unchecked()` here.
                    out.set_elt(i, v.as_ref())?;

                    last_index = i;
                }

                let new_len = last_index + 1;
                if new_len != upper {
                    unsafe {
                        savvy_ffi::SETLENGTH(out.inner, new_len as _);
                    }
                    out.len = new_len;
                }

                Ok(out)
            }
            (_, None) => {
                // When the length is not known at all, collect() it first.

                let v: Vec<I::Item> = iter.collect();
                v.try_into()
            }
        }
    }

    /// Constructs a new string vector from a slice or vec.
    pub fn try_from_slice<S, U>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[U]>,
        U: AsRef<str>,
    {
        let x_slice = x.as_ref();
        let mut out = Self::new(x_slice.len())?;
        for (i, v) in x_slice.iter().enumerate() {
            // Safety: slice and OwnedStringSexp have the same length.
            unsafe { out.set_elt_unchecked(i, str_to_charsxp(v.as_ref())?) };
        }
        Ok(out)
    }

    /// Constructs a new string vector from a scalar value.
    pub fn try_from_scalar<T: AsRef<str>>(value: T) -> crate::error::Result<Self> {
        let sexp = unsafe {
            // Note: unlike `new()`, this allocates a STRSXP after creating a
            // CHARSXP. So, the `CHARSXP` needs to be protected.
            let charsxp = str_to_charsxp(value.as_ref())?;
            savvy_ffi::Rf_protect(charsxp);
            let out = crate::unwind_protect(|| savvy_ffi::Rf_ScalarString(charsxp))?;
            savvy_ffi::Rf_unprotect(1);
            out
        };
        Self::new_from_raw_sexp(sexp, 1)
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
        value.assert_string()?;
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
        Self::try_from_slice(value)
    }
}

impl<T> TryFrom<Vec<T>> for OwnedStringSexp
where
    T: AsRef<str>, // This works both for &str and String
{
    type Error = crate::error::Error;

    fn try_from(value: Vec<T>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<&str> for OwnedStringSexp {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
    }
}

impl TryFrom<String> for OwnedStringSexp {
    type Error = crate::error::Error;

    fn try_from(value: String) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
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

impl<'a> ExactSizeIterator for StringSexpIter<'a> {}
