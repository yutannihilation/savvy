use std::ops::{Index, IndexMut};

use libR_sys::{Rf_xlength, INTEGER, INTEGER_ELT, INTSXP, SEXP};

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
    inner: SEXP,
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

    pub fn as_slice(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(INTEGER(self.inner()) as _, self.len()) }
    }

    /// If the input is an ALTREP, this materialize it first, so it might not be
    /// most efficient. However, it seems Rust's slice implementation is very
    /// fast, so probably being efficient for ALTREP is not worth giving up the
    /// benefit.
    pub fn iter(&self) -> std::slice::Iter<i32> {
        self.as_slice().iter()
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::with_capacity(self.len());
        out.copy_from_slice(self.as_slice());
        out
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
        self.len == 0
    }

    pub fn as_read_only(&self) -> IntegerSxp {
        IntegerSxp(self.inner)
    }

    pub fn as_slice(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    pub fn iter(&self) -> std::slice::Iter<i32> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<i32> {
        self.as_mut_slice().iter_mut()
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::with_capacity(self.len());
        out.copy_from_slice(self.as_slice());
        out
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: i32) {
        self[i] = v;
    }

    pub fn new(len: usize) -> crate::Result<Self> {
        let inner = crate::alloc_vector(INTSXP, len as _)?;
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { INTEGER(inner) };

        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }
}

impl Drop for OwnedIntegerSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for IntegerSxp {
    type Error = crate::Error;

    fn try_from(value: Sxp) -> crate::Result<Self> {
        if !value.is_integer() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to integer");
            return Err(crate::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl TryFrom<&[i32]> for OwnedIntegerSxp {
    type Error = crate::error::Error;

    fn try_from(value: &[i32]) -> crate::Result<Self> {
        let mut out = Self::new(value.len())?;
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
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
    pub sexp: &'a SEXP,
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
            Some(unsafe { INTEGER_ELT(*self.sexp, i as _) })
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i))) }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for IntegerSxpIter<'a> {
    fn len(&self) -> usize {
        self.len
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
        if index >= self.len {
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
        if index >= self.len {
            panic!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, index
            );
        }
        unsafe { &mut *(self.raw.add(index)) }
    }
}
