use std::ops::{Index, IndexMut};

use num_complex::Complex64;
use savvy_ffi::CPLXSXP;
use savvy_ffi::{COMPLEX, SEXP};

use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
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
        super::utils::verify_len(self.len, i)?;

        unsafe {
            *(self.raw.add(i)) = v;
        }

        Ok(())
    }

    /// Set the `i`-th element to NA.
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        super::utils::verify_len(self.len, i)?;

        unsafe {
            *(self.raw.add(i)) = Complex64::na();
        }

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
        value.verify_complex()?;
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
        let mut out = unsafe { Self::new_without_init(value.len())? };
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
    }
}

impl TryFrom<Vec<Complex64>> for OwnedComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<Complex64>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<Complex64> for OwnedComplexSexp {
    type Error = crate::error::Error;

    fn try_from(value: Complex64) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarComplex(value))? };
        Self::new_from_raw_sexp(sexp, 1)
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
        if index >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedComplexSexp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &mut *(self.raw.add(index)) }
    }
}
