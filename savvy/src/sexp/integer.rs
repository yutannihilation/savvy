use std::ops::{Index, IndexMut};

use libR_sys::{Rf_allocVector, Rf_xlength, ALTREP, INTEGER, INTEGER_ELT, INTSXP, SEXP};

use super::Sxp;
use crate::protect;

// This is based on the idea of cpp11's `writable`.
//
// `IntegerSxp` is a read-only wrapper for SEXPs provided from outside of Rust;
// since it's the caller's responsibility to PROTECT it, we don't protect it on
// Rust's side.
//
// `OwnedIntegerSxp` is a writable wrapper for SEXPs newly allocated on Rust's
// side. Since it's us who produce it, we protect it and drop it.
pub struct IntegerSxp(pub SEXP);
pub struct OwnedIntegerSxp {
    pub inner: IntegerSxp,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

impl IntegerSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> IntegerSxpIter {
        // if the vector is an ALTREP, we cannot directly access the underlying
        // data.
        let raw = unsafe {
            if ALTREP(self.0) == 1 {
                std::ptr::null()
            } else {
                INTEGER(self.0)
            }
        };

        IntegerSxpIter {
            sexp: self,
            raw,
            i: 0,
            len: self.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedIntegerSxp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> IntegerSxpIter {
        IntegerSxpIter {
            sexp: &self.inner,
            raw: self.raw,
            i: 0,
            len: self.len,
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.inner.to_vec()
    }

    pub fn as_slice(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(INTEGER(self.inner()) as _, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(INTEGER(self.inner()) as _, self.len) }
    }

    pub fn inner(&self) -> SEXP {
        self.inner.inner()
    }

    pub fn set_elt(&mut self, i: usize, v: i32) {
        self[i] = v;
    }

    pub fn new(len: usize) -> Self {
        let out = unsafe { Rf_allocVector(INTSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);
        let raw = unsafe { INTEGER(out) };

        Self {
            inner: IntegerSxp(out),
            token,
            len,
            raw,
        }
    }
}

impl Drop for OwnedIntegerSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for IntegerSxp {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        if !value.is_integer() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to integer");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<IntegerSxp> for SEXP {
    fn from(value: IntegerSxp) -> Self {
        value.inner()
    }
}

impl From<OwnedIntegerSxp> for SEXP {
    fn from(value: OwnedIntegerSxp) -> Self {
        value.inner()
    }
}

pub struct IntegerSxpIter<'a> {
    pub sexp: &'a IntegerSxp,
    raw: *const i32,
    i: usize,
    len: usize,
}

impl<'a> Iterator for IntegerSxpIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        if self.raw.is_null() {
            // When ALTREP, access to the value via *_ELT()
            Some(unsafe { INTEGER_ELT(self.sexp.0, i as _) })
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i))) }
        }
    }
}

// You might also want to write something like:
//
// ```rust
// let sxp: OwnedIntegerSxp = i.map(...).collect();
// ```
//
// But, this is inefficient in that this allocates twice just because the length
// is not known inside `FromIterator` even when you know the actual length.
//
// ```rust
// impl FromIterator<i32> for OwnedIntegerSxp {
//     fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
//         // In order to know the length, this has to be collected first.
//         let v: Vec<i32> = iter.into_iter().collect();

//         let mut out = Self::new(v.len());
//         v.into_iter()
//             .enumerate()
//             .for_each(|(i, v)| out.set_elt(i, v));
//         out
//     }
// }
// ```

impl Index<usize> for OwnedIntegerSxp {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedIntegerSxp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &mut *(self.raw.add(index)) }
    }
}
