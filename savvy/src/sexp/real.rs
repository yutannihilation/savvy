use std::ops::{Index, IndexMut};

use savvy_ffi::{Rf_xlength, REAL, REALSXP, SEXP};

use super::Sexp;
use crate::protect;

pub struct RealSexp(pub SEXP);
pub struct OwnedRealSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut f64,
}

impl RealSexp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(REAL(self.0), self.len()) }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.as_slice().iter()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.len());
        out.copy_from_slice(self.as_slice());
        out
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedRealSexp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> RealSexp {
        RealSexp(self.inner)
    }

    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<f64> {
        self.as_mut_slice().iter_mut()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.len);
        out.copy_from_slice(self.as_slice());
        out
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: f64) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }

        unsafe {
            *(self.raw.add(i)) = v;
        }

        Ok(())
    }

    pub fn new(len: usize) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(REALSXP, len as _)?;
        Self::new_from_raw_sexp(inner, len)
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
        if !value.is_real() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to real");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
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
        let mut out = Self::new(value.len())?;
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
    }
}

impl TryFrom<Vec<f64>> for OwnedRealSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<f64>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<f64> for OwnedRealSexp {
    type Error = crate::error::Error;

    fn try_from(value: f64) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarReal(value))? };
        Self::new_from_raw_sexp(sexp, 1)
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
        if index >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedRealSexp {
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
