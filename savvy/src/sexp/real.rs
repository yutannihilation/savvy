use std::ops::{Index, IndexMut};

use libR_sys::{Rf_xlength, REAL, REALSXP, SEXP};

use super::Sxp;
use crate::protect;

pub struct RealSxp(pub SEXP);
pub struct OwnedRealSxp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut f64,
}

impl RealSxp {
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

impl OwnedRealSxp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> RealSxp {
        RealSxp(self.inner)
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

    pub fn set_elt(&mut self, i: usize, v: f64) {
        self[i] = v;
    }

    pub fn new(len: usize) -> crate::Result<Self> {
        let inner = crate::alloc_vector(REALSXP, len as _)?;
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

impl Drop for OwnedRealSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for RealSxp {
    type Error = crate::Error;

    fn try_from(value: Sxp) -> crate::Result<Self> {
        if !value.is_real() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to real");
            return Err(crate::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl TryFrom<&[f64]> for OwnedRealSxp {
    type Error = crate::Error;

    fn try_from(value: &[f64]) -> crate::Result<Self> {
        let mut out = Self::new(value.len())?;
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<RealSxp> for SEXP {
    fn from(value: RealSxp) -> Self {
        value.inner()
    }
}

impl From<OwnedRealSxp> for SEXP {
    fn from(value: OwnedRealSxp) -> Self {
        value.inner()
    }
}

impl Index<usize> for OwnedRealSxp {
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

impl IndexMut<usize> for OwnedRealSxp {
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
