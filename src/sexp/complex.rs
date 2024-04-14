use std::ops::{Index, IndexMut};

use num_complex::Complex64;
use savvy_ffi::CPLXSXP;
use savvy_ffi::{COMPLEX, SEXP};

use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, utils::assert_len, Sexp};
use crate::protect;
use crate::NotAvailableValue; // for na()

/// An external SEXP of a complex vector.
pub struct ComplexSexp(pub SEXP);

/// A newly-created SEXP of a complex vector
pub struct OwnedComplexSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut Complex64,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(ComplexSexp);
impl_common_sexp_ops_owned!(OwnedComplexSexp);

impl ComplexSexp {
    pub fn as_slice(&self) -> &[Complex64] {
        unsafe { std::slice::from_raw_parts(COMPLEX(self.inner()) as _, self.len()) }
    }

    pub fn iter(&self) -> std::slice::Iter<Complex64> {
        self.as_slice().iter()
    }

    pub fn to_vec(&self) -> Vec<Complex64> {
        self.as_slice().to_vec()
    }
}

impl OwnedComplexSexp {
    pub fn as_read_only(&self) -> ComplexSexp {
        ComplexSexp(self.inner)
    }

    pub fn as_slice(&self) -> &[Complex64] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [Complex64] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    pub fn iter(&self) -> std::slice::Iter<Complex64> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Complex64> {
        self.as_mut_slice().iter_mut()
    }

    pub fn to_vec(&self) -> Vec<Complex64> {
        self.as_slice().to_vec()
    }

    /// Set the value of the `i`-th element.
    pub fn set_elt(&mut self, i: usize, v: Complex64) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, v) };

        Ok(())
    }

    #[inline]
    unsafe fn set_elt_unchecked(&mut self, i: usize, v: Complex64) {
        unsafe { *(self.raw.add(i)) = v };
    }

    /// Set the `i`-th element to NA.
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, Complex64::na()) };

        Ok(())
    }

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(CPLXSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(COMPLEX(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    /// Constructs a new, initialized integer vector.
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// # Safety
    ///
    /// As the memory is uninitialized, all elements must be filled values
    /// before return.
    pub unsafe fn new_without_init(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, false)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { COMPLEX(inner) };

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
    pub fn try_from_iter<I>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = Complex64>,
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

                if last_index + 1 != upper {
                    unsafe {
                        savvy_ffi::SETLENGTH(out.inner, (last_index + 1) as _);
                    }
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

    /// Constructs a new complex vector from a slice or vec.
    pub fn try_from_slice<S>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[Complex64]>,
    {
        let x_slice = x.as_ref();
        let mut out = unsafe { Self::new_without_init(x_slice.len())? };
        out.as_mut_slice().copy_from_slice(x_slice);
        Ok(out)
    }

    /// Constructs a new integer vector from a scalar value.
    pub fn try_from_scalar(value: Complex64) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarComplex(value))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

impl Drop for OwnedComplexSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to ComplexSexp ***************

impl TryFrom<Sexp> for ComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_complex()?;
        Ok(Self(value.0))
    }
}

impl From<ComplexSexp> for Sexp {
    fn from(value: ComplexSexp) -> Self {
        Self(value.inner())
    }
}

impl From<ComplexSexp> for crate::error::Result<Sexp> {
    fn from(value: ComplexSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedComplexSexp ***************

impl TryFrom<&[Complex64]> for OwnedComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[Complex64]) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<Vec<Complex64>> for OwnedComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<Complex64>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<Complex64> for OwnedComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: Complex64) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
    }
}

impl From<OwnedComplexSexp> for Sexp {
    fn from(value: OwnedComplexSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedComplexSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedComplexSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

macro_rules! impl_try_from_rust_complexes {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedComplexSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_complexes!(&[Complex64]);
impl_try_from_rust_complexes!(Vec<Complex64>);
impl_try_from_rust_complexes!(Complex64);

// Index for OwnedComplexSexp ***************

impl Index<usize> for OwnedComplexSexp {
    type Output = Complex64;

    fn index(&self, index: usize) -> &Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedComplexSexp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &mut *(self.raw.add(index)) }
    }
}
