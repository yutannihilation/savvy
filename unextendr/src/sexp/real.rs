use libR_sys::{Rf_allocVector, Rf_xlength, ALTREP, REAL, REALSXP, REAL_ELT, SET_REAL_ELT, SEXP};

use super::Sxp;
use crate::{error::get_human_readable_type_name, protect};

pub struct RealSxp(SEXP);
pub struct OwnedRealSxp {
    inner: RealSxp,
    token: SEXP,
}

impl RealSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> f64 {
        unsafe { REAL_ELT(self.0, i as _) }
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
            sexp: self,
            raw,
            i: 0,
            len: self.len(),
        }
    }

    fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedRealSxp {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn elt(&self, i: usize) -> f64 {
        self.inner.elt(i)
    }

    pub fn iter(&self) -> RealSxpIter {
        self.inner.iter()
    }

    pub(crate) fn inner(&self) -> SEXP {
        self.inner.inner()
    }

    pub fn set_elt(&mut self, i: usize, v: f64) {
        unsafe {
            SET_REAL_ELT(self.inner(), i as _, v);
        }
    }

    pub fn new(len: usize) -> Self {
        let out = unsafe { Rf_allocVector(REALSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);
        Self {
            inner: RealSxp(out),
            token,
        }
    }
}

impl Drop for OwnedRealSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<SEXP> for RealSxp {
    type Error = crate::error::Error;

    fn try_from(value: SEXP) -> crate::error::Result<Self> {
        if !Sxp(value).is_real() {
            let type_name = get_human_readable_type_name(value);
            let msg = format!("Cannot convert {type_name} to real");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value))
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

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for RealSxp {
//         type Output = f64;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct RealSxpIter<'a> {
    pub sexp: &'a RealSxp,
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
            Some(self.sexp.elt(i))
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i))) }
        }
    }
}
