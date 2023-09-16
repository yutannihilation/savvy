use libR_sys::{
    Rf_allocVector, Rf_xlength, ALTREP, LGLSXP, LOGICAL, LOGICAL_ELT, SET_LOGICAL_ELT, SEXP,
};

use super::Sxp;
use crate::protect;

pub struct LogicalSxp(pub SEXP);
pub struct OwnedLogicalSxp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

impl LogicalSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> LogicalSxpIter {
        // if the vector is an ALTREP, we cannot directly access the underlying
        // data.
        let raw = unsafe {
            if ALTREP(self.0) == 1 {
                std::ptr::null()
            } else {
                LOGICAL(self.0)
            }
        };

        LogicalSxpIter {
            sexp: &self.0,
            raw,
            i: 0,
            len: self.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedLogicalSxp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> LogicalSxp {
        LogicalSxp(self.inner)
    }

    pub fn iter(&self) -> LogicalSxpIter {
        LogicalSxpIter {
            sexp: &self.inner,
            raw: self.raw,
            i: 0,
            len: self.len,
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: bool) {
        if i >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            );
        }
        unsafe {
            SET_LOGICAL_ELT(self.inner, i as _, v as _);
        }
    }

    pub fn new(len: usize) -> Self {
        let inner = unsafe { Rf_allocVector(LGLSXP, len as _) };
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { LOGICAL(inner) };
        Self {
            inner,
            token,
            len,
            raw,
        }
    }
}

impl Drop for OwnedLogicalSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for LogicalSxp {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        if !value.is_logical() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to logical");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<&[bool]> for OwnedLogicalSxp {
    fn from(value: &[bool]) -> Self {
        let mut out = Self::new(value.len());
        value
            .iter()
            .enumerate()
            .for_each(|(i, v)| out.set_elt(i, *v));
        out
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<LogicalSxp> for SEXP {
    fn from(value: LogicalSxp) -> Self {
        value.inner()
    }
}

impl From<OwnedLogicalSxp> for SEXP {
    fn from(value: OwnedLogicalSxp) -> Self {
        value.inner()
    }
}

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for LogicalSxp {
//         type Output = i32;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct LogicalSxpIter<'a> {
    pub sexp: &'a SEXP,
    raw: *const i32,
    i: usize,
    len: usize,
}

impl<'a> Iterator for LogicalSxpIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        if self.raw.is_null() {
            // When ALTREP, access to the value via *_ELT()
            Some(unsafe { LOGICAL_ELT(*self.sexp, i as _) } == 1)
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i)) == 1) }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for LogicalSxpIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}
