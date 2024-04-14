use savvy_ffi::{R_NaInt, LGLSXP, LOGICAL, SET_LOGICAL_ELT, SEXP};

use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect;

/// An external SEXP of a logical vector.
pub struct LogicalSexp(pub SEXP);

/// A newly-created SEXP of a logical vector.
pub struct OwnedLogicalSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(LogicalSexp);
impl_common_sexp_ops_owned!(OwnedLogicalSexp);

impl LogicalSexp {
    /// Returns the internal representation, **`&[i32]`, not `&[bool]`**. This
    /// is an expert-only function which might be found useful when you really
    /// need to distinguish NAs.
    pub fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(LOGICAL(self.0), self.len()) }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// # let lgl_sexp = savvy::OwnedLogicalSexp::try_from_slice([true, true, false])?.as_read_only();
    /// // `lgl_sexp` is c(TRUE, TRUE, FALSE)
    /// let mut iter = lgl_sexp.iter();
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.collect::<Vec<bool>>(), vec![true, false]);
    /// ```
    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    /// Copies the underlying data of the SEXP into a new `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// # let lgl_sexp = savvy::OwnedLogicalSexp::try_from_slice([true, true, false])?.as_read_only();
    /// // `lgl_sexp` is c(TRUE, TRUE, FALSE)
    /// assert_eq!(lgl_sexp.to_vec(), vec![true, true, false]);
    /// ```
    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }
}

impl OwnedLogicalSexp {
    /// Returns the read-only version of the wrapper. This is mainly for testing
    /// purposes.
    pub fn as_read_only(&self) -> LogicalSexp {
        LogicalSexp(self.inner)
    }

    /// Returns the internal representation, `&[i32]`, not `&[bool]`. This is an
    /// expert-only function which might be found useful when you really need to
    /// distinguish NAs.
    pub fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len()) }
    }

    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    /// Set the value of the `i`-th element.
    pub fn set_elt(&mut self, i: usize, v: bool) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, v as _) };

        Ok(())
    }

    // Set the value of the `i`-th element.
    // Safety: the user has to assure bounds are checked.
    #[inline]
    unsafe fn set_elt_unchecked(&mut self, i: usize, v: i32) {
        unsafe { SET_LOGICAL_ELT(self.inner, i as _, v) };
    }

    /// Set the `i`-th element to NA.
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, R_NaInt) };

        Ok(())
    }

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(LGLSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(LOGICAL(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    /// Constructs a new, initialized logical vector.
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// Constructs a new, **uninitialized** logical vector.
    ///
    /// This is an expert-only version of `new()`, which can be found useful
    /// when you want to skip initialization and you are confident that the
    /// vector will be filled with values later.
    ///
    /// For example, you can use this in `TryFrom` implementation.
    ///
    /// ``` no_run
    /// struct Pair {
    ///     x: bool,
    ///     y: bool
    /// }
    ///
    /// impl TryFrom<Pair> for Sexp {
    ///     type Error = savvy::Error;
    ///
    ///     fn try_from(value: Pair) -> savvy::Result<Self> {
    ///         let mut out = unsafe { OwnedLogicalSexp::new_without_init(2)? };
    ///         out.set_elt(0, value.x)?;
    ///         out.set_elt(1, value.x)?;
    ///         
    ///         out.into()
    ///     }
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// As the memory is uninitialized, all elements must be filled values
    /// before return.
    pub unsafe fn new_without_init(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, false)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { LOGICAL(inner) };

        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }

    /// Constructs a new logical vector from an iterator.
    ///
    /// Note that, if you already have a slice or vec, you can also use
    /// [`try_from_slice`][1].
    ///
    /// [1]: `Self::try_from_slice()`
    pub fn try_from_iter<I>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = bool>,
    {
        let iter = iter.into_iter();

        match iter.size_hint() {
            (_, Some(upper)) => {
                // If the maximum length is known, use it at frist. But, the
                // iterator's length might be shorter than the reported one
                // (e.g. `(0..10).filter(|x| x % 2 == 0)`), so it needs to be
                // truncated to the actual length at last.

                let mut out = unsafe { Self::new_without_init(upper)? };

                let mut last_index = 0;
                for (i, v) in iter.enumerate() {
                    // The upper bound of size_hint() is just for optimization
                    // and what we should not trust. So, we should't use
                    // `set_elt_unchecked()` here.
                    out.set_elt(i, v)?;

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

    /// Constructs a new logical vector from a slice or vec.
    pub fn try_from_slice<S>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[bool]>,
    {
        let x_slice = x.as_ref();
        let mut out = unsafe { Self::new_without_init(x_slice.len())? };
        for (i, v) in x_slice.iter().enumerate() {
            // Safety: slice and OwnedLogicalSexp have the same length.
            unsafe { out.set_elt_unchecked(i, *v as _) };
        }
        Ok(out)
    }

    /// Constructs a new logical vector from a scalar value.
    pub fn try_from_scalar(value: bool) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarLogical(value as i32))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

impl Drop for OwnedLogicalSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to LogicalSexp ***************

impl TryFrom<Sexp> for LogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_logical()?;
        Ok(Self(value.0))
    }
}

impl From<LogicalSexp> for Sexp {
    fn from(value: LogicalSexp) -> Self {
        Self(value.inner())
    }
}

impl From<LogicalSexp> for crate::error::Result<Sexp> {
    fn from(value: LogicalSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedLogicalSexp ***************

impl TryFrom<&[bool]> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[bool]) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<Vec<bool>> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<bool>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<bool> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: bool) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
    }
}

macro_rules! impl_try_from_rust_reals {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedLogicalSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_reals!(&[bool]);
impl_try_from_rust_reals!(Vec<bool>);
impl_try_from_rust_reals!(bool);

impl From<OwnedLogicalSexp> for Sexp {
    fn from(value: OwnedLogicalSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedLogicalSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedLogicalSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// Index for OwnedIntegerSexp ***************

pub struct LogicalSexpIter<'a> {
    iter_raw: std::slice::Iter<'a, i32>,
}

impl<'a> Iterator for LogicalSexpIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_raw.next().map(|x| *x == 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter_raw.size_hint()
    }
}

impl<'a> ExactSizeIterator for LogicalSexpIter<'a> {}
