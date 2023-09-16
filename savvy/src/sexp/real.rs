use std::ops::{Index, IndexMut};

use libR_sys::{Rf_allocVector, Rf_xlength, ALTREP, REAL, REALSXP, REAL_ELT, SEXP};

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

    pub fn iter(&self) -> RealSxpIter {
        // if the vector is an ALTREP, we cannot directly access the underlying
        // data.
        let raw = unsafe {
            if ALTREP(self.0) == 1 {
                std::ptr::null()
            } else {
                REAL(self.0)
            }
        };

        RealSxpIter {
            sexp: &self.0,
            raw,
            i: 0,
            len: self.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.iter().collect()
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

    pub fn iter(&self) -> RealSxpIter {
        RealSxpIter {
            sexp: &self.inner,
            raw: self.raw,
            i: 0,
            len: self.len,
        }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.iter().collect()
    }

    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: f64) {
        self[i] = v;
    }

    pub fn new(len: usize) -> Self {
        let inner = unsafe { Rf_allocVector(REALSXP, len as _) };
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { REAL(inner) };

        Self {
            inner,
            token,
            len,
            raw,
        }
    }
}

impl Drop for OwnedRealSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for RealSxp {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        if !value.is_real() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to real");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<&[f64]> for OwnedRealSxp {
    fn from(value: &[f64]) -> Self {
        let mut out = Self::new(value.len());
        out.as_mut_slice().copy_from_slice(value);
        out
    }
}

// This conflicts...
//
// impl<I> From<I> for OwnedRealSxp
// where
//     I: ExactSizeIterator + Iterator<Item = f64>,
// {
//     fn from(value: I) -> Self {
//         let mut out = Self::new(value.len());
//         value.enumerate().for_each(|(i, v)| out.set_elt(i, v));
//         out
//     }
// }

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

pub struct RealSxpIter<'a> {
    pub sexp: &'a SEXP,
    raw: *const f64,
    i: usize,
    len: usize,
}

impl<'a> Iterator for RealSxpIter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        if self.raw.is_null() {
            // When ALTREP, access to the value via *_ELT()
            Some(unsafe { REAL_ELT(*self.sexp, i as _) })
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i))) }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for RealSxpIter<'a> {
    fn len(&self) -> usize {
        self.len
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
