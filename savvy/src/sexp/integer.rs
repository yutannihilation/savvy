use std::ops::{Index, IndexMut};

use savvy_ffi::{Rf_xlength, INTEGER, INTSXP, SEXP};

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

    pub fn set_elt(&mut self, i: usize, v: i32) -> crate::error::Result<()> {
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
        let inner = crate::alloc_vector(INTSXP, len as _)?;
        Self::new_from_raw_sexp(inner, len)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
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

impl TryFrom<&[i32]> for OwnedIntegerSxp {
    type Error = crate::error::Error;

    fn try_from(value: &[i32]) -> crate::error::Result<Self> {
        let mut out = Self::new(value.len())?;
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
    }
}

impl TryFrom<i32> for OwnedIntegerSxp {
    type Error = crate::error::Error;

    fn try_from(value: i32) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarInteger(value))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<IntegerSxp> for Sxp {
    fn from(value: IntegerSxp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedIntegerSxp> for Sxp {
    fn from(value: OwnedIntegerSxp) -> Self {
        Self(value.inner())
    }
}

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
