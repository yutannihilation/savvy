use std::ops::{Index, IndexMut};

use savvy_ffi::{REAL, REALSXP, SEXP};

use super::utils::assert_len;
use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect::{self, local_protect};
use crate::NotAvailableValue; // for na()

/// An external SEXP of a real vector.
pub struct RealSexp(pub SEXP);

/// A newly-created SEXP of a real vector.
pub struct OwnedRealSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut f64,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(RealSexp);
impl_common_sexp_ops_owned!(OwnedRealSexp);

impl RealSexp {
    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// # let real_sexp = savvy::OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?.as_read_only();
    /// // `real_sexp` is c(1.0, 2.0, 3.0)
    /// assert_eq!(real_sexp.as_slice(), &[1.0, 2.0, 3.0]);
    /// ```
    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(REAL(self.0), self.len()) }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// # let real_sexp = savvy::OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?.as_read_only();
    /// // `real_sexp` is c(1.0, 2.0, 3.0)
    /// let mut iter = real_sexp.iter();
    /// assert_eq!(iter.next(), Some(&1.0));
    /// assert_eq!(iter.as_slice(), &[2.0, 3.0]);
    /// ```
    ///
    /// # Technical Note
    ///
    /// If the input is an ALTREP, this materialize it first, so it might not be
    /// most efficient. However, it seems Rust's slice implementation is very
    /// fast, so probably being efficient for ALTREP is not worth giving up the
    /// benefit.
    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.as_slice().iter()
    }

    /// Copies the underlying data of the SEXP into a new `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// # let real_sexp = savvy::OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?.as_read_only();
    /// // `real_sexp` is c(1.0, 2.0, 3.0)
    /// assert_eq!(real_sexp.to_vec(), vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn to_vec(&self) -> Vec<f64> {
        self.as_slice().to_vec()
    }
}

impl OwnedRealSexp {
    /// Returns the read-only version of the wrapper. This is mainly for testing
    /// purposes.
    pub fn as_read_only(&self) -> RealSexp {
        RealSexp(self.inner)
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let real_sexp = OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?;
    /// assert_eq!(real_sexp.to_vec(), vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    /// Extracts a mutable slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let mut real_sexp = OwnedRealSexp::new(3)?;
    /// let s = real_sexp.as_mut_slice();
    /// s[2] = 10.0;
    /// assert_eq!(real_sexp.as_slice(), &[0.0, 0.0, 10.0]);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let mut real_sexp = OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?;
    /// real_sexp.iter_mut().for_each(|x| *x = *x * 2.0);
    /// assert_eq!(real_sexp.as_slice(), &[2.0, 4.0, 6.0]);
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<f64> {
        self.as_mut_slice().iter_mut()
    }

    /// Copies the underlying data of the SEXP into a new `Vec`.
    pub fn to_vec(&self) -> Vec<f64> {
        self.as_slice().to_vec()
    }

    /// Set the value of the `i`-th element. `i` starts from `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let mut real_sexp = OwnedRealSexp::new(3)?;
    /// real_sexp.set_elt(2, 10.0)?;
    /// assert_eq!(real_sexp.as_slice(), &[0.0, 0.0, 10.0]);
    /// ```
    pub fn set_elt(&mut self, i: usize, v: f64) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, v) };

        Ok(())
    }

    #[inline]
    unsafe fn set_elt_unchecked(&mut self, i: usize, v: f64) {
        unsafe { *(self.raw.add(i)) = v };
    }

    /// Set the `i`-th element to NA. `i` starts from `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    /// use savvy::NotAvailableValue;
    ///
    /// let mut real_sexp = OwnedRealSexp::new(3)?;
    /// real_sexp.set_na(2)?;
    ///
    /// // R's NA is one of NaN values, and cannot be easily asserted
    /// // (e.g. NaN == NaN is false).
    /// assert!(real_sexp[2].is_na());
    /// ```
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, f64::na()) };

        Ok(())
    }

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(REALSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(REAL(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    /// Constructs a new, initialized real vector.
    ///
    /// ```
    /// let x = savvy::OwnedRealSexp::new(3)?;
    /// assert_eq!(x.as_slice(), &[0.0, 0.0, 0.0]);
    /// ```
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// Constructs a new, **uninitialized** real vector.
    ///
    /// This is an expert-only version of `new()`, which can be found useful
    /// when you want to skip initialization and you are confident that the
    /// vector will be filled with values later.
    ///
    /// For example, you can use this in `TryFrom` implementation.
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// struct Pair {
    ///     x: f64,
    ///     y: f64
    /// }
    ///
    /// impl TryFrom<Pair> for OwnedRealSexp {
    ///     type Error = savvy::Error;
    ///
    ///     fn try_from(value: Pair) -> savvy::Result<Self> {
    ///         let mut out = unsafe { OwnedRealSexp::new_without_init(2)? };
    ///         out[0] = value.x;
    ///         out[1] = value.y;
    ///
    ///         Ok(out)
    ///     }
    /// }
    ///
    /// let pair = Pair { x: 1.0, y: 2.0 };
    /// let real_sexp = <OwnedRealSexp>::try_from(pair)?;
    /// assert_eq!(real_sexp.as_slice(), &[1.0, 2.0]);
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
        let raw = unsafe { REAL(inner) };

        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }

    /// Constructs a new complex vector from an iterator.
    ///
    /// Note that, if you already have a slice or vec, [`try_from_slice()`][1]
    /// is what you want. `try_from_slice` is more performant than
    /// `try_from_iter` because it copies the underlying memory directly.
    ///
    /// [1]: `Self::try_from_slice()`
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let iter = [-2.0, 0.0, 2.0, -5.0, 3.0].into_iter().filter(|x| *x > 0.0);
    /// let real_sexp = OwnedRealSexp::try_from_iter(iter)?;
    /// assert_eq!(real_sexp.as_slice(), &[2.0, 3.0]);
    /// ```
    pub fn try_from_iter<I>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = f64>,
    {
        let iter = iter.into_iter();

        match iter.size_hint() {
            (_, Some(upper)) => {
                // If the maximum length is known, use it at frist. But, the
                // iterator's length might be shorter than the reported one
                // (e.g. `(0..10).filter(|x| x % 2 == 0)`), so it needs to be
                // truncated to the actual length at last.

                let inner = crate::alloc_vector(REALSXP, upper as _)?;
                let _inner_guard = local_protect(inner);
                let raw = unsafe { REAL(inner) };

                let mut last_index = 0;
                for (i, v) in iter.enumerate() {
                    // The upper bound of size_hint() is just for optimization
                    // and what we should not trust.
                    assert_len(upper, i)?;
                    unsafe { *(raw.add(i)) = v };

                    last_index = i;
                }

                let new_len = last_index + 1;
                if new_len == upper {
                    // If the length is the same as expected, use it as it is.
                    Self::new_from_raw_sexp(inner, upper)
                } else {
                    // If the length is shorter than expected, re-allocate a new
                    // SEXP and copy the values into it.
                    let out = unsafe { Self::new_without_init(new_len)? };
                    let dst = unsafe { std::slice::from_raw_parts_mut(out.raw, new_len) };
                    // `raw` is longer than new_len, but the elements over new_len are ignored
                    let src = unsafe { std::slice::from_raw_parts(raw, new_len) };
                    dst.copy_from_slice(src);

                    Ok(out)
                }
            }
            (_, None) => {
                // When the length is not known at all, collect() it first.

                let v: Vec<I::Item> = iter.collect();
                v.try_into()
            }
        }
    }

    /// Constructs a new real vector from a slice or vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let real_sexp = OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?;
    /// assert_eq!(real_sexp.as_slice(), &[1.0, 2.0, 3.0]);
    /// ```
    pub fn try_from_slice<S>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[f64]>,
    {
        let x_slice = x.as_ref();
        let mut out = unsafe { Self::new_without_init(x_slice.len())? };
        out.as_mut_slice().copy_from_slice(x_slice);
        Ok(out)
    }

    /// Constructs a new integer vector from a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRealSexp;
    ///
    /// let real_sexp = OwnedRealSexp::try_from_scalar(1.0)?;
    /// assert_eq!(real_sexp.as_slice(), &[1.0]);
    /// ```
    pub fn try_from_scalar(value: f64) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarReal(value))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

impl Drop for OwnedRealSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to RealSexp ***************

impl TryFrom<Sexp> for RealSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_real()?;
        Ok(Self(value.0))
    }
}

impl From<RealSexp> for Sexp {
    fn from(value: RealSexp) -> Self {
        Self(value.inner())
    }
}

impl From<RealSexp> for crate::error::Result<Sexp> {
    fn from(value: RealSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedRealSexp ***************

impl TryFrom<&[f64]> for OwnedRealSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[f64]) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<Vec<f64>> for OwnedRealSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<f64>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<f64> for OwnedRealSexp {
    type Error = crate::error::Error;

    fn try_from(value: f64) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
    }
}

impl From<OwnedRealSexp> for Sexp {
    fn from(value: OwnedRealSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedRealSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedRealSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

macro_rules! impl_try_from_rust_reals {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedRealSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_reals!(&[f64]);
impl_try_from_rust_reals!(Vec<f64>);
impl_try_from_rust_reals!(f64);

// Index for OwnedRealSexp ***************

impl Index<usize> for OwnedRealSexp {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedRealSexp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &mut *(self.raw.add(index)) }
    }
}
