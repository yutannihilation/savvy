use std::ops::{Index, IndexMut};

use savvy_ffi::{INTEGER, INTSXP, SEXP};

use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect;

// This is based on the idea of cpp11's `writable`.
//
// `IntegerSexp` is a read-only wrapper for SEXPs provided from outside of Rust;
// since it's the caller's responsibility to PROTECT it, we don't protect it on
// Rust's side.
//
// `OwnedIntegerSexp` is a writable wrapper for SEXPs newly allocated on Rust's
// side. Since it's us who produce it, we protect it and drop it.
pub struct IntegerSexp(pub SEXP);
pub struct OwnedIntegerSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(IntegerSexp);
impl_common_sexp_ops_owned!(OwnedIntegerSexp);

impl IntegerSexp {
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
}

impl OwnedIntegerSexp {
    pub fn as_read_only(&self) -> IntegerSexp {
        IntegerSexp(self.inner)
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

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(INTSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(INTEGER(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// # Safety
    ///
    /// This is an expert-only version of `new()` in case the user needs to skip
    /// the initialization for some great purpose.
    pub unsafe fn new_without_init(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
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

impl Drop for OwnedIntegerSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to IntegerSexp ***************

impl TryFrom<Sexp> for IntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_integer() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected integers, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<IntegerSexp> for Sexp {
    fn from(value: IntegerSexp) -> Self {
        Self(value.inner())
    }
}

impl From<IntegerSexp> for crate::error::Result<Sexp> {
    fn from(value: IntegerSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedIntegerSexp ***************

impl TryFrom<&[i32]> for OwnedIntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[i32]) -> crate::error::Result<Self> {
        let mut out = unsafe { Self::new_without_init(value.len())? };
        out.as_mut_slice().copy_from_slice(value);
        Ok(out)
    }
}

impl TryFrom<Vec<i32>> for OwnedIntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<i32>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<i32> for OwnedIntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: i32) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarInteger(value))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

impl From<OwnedIntegerSexp> for Sexp {
    fn from(value: OwnedIntegerSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedIntegerSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedIntegerSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

macro_rules! impl_try_from_rust_integers {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedIntegerSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_integers!(&[i32]);
impl_try_from_rust_integers!(Vec<i32>);
impl_try_from_rust_integers!(i32);

// Index for OwnedIntegerSexp ***************

impl Index<usize> for OwnedIntegerSexp {
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

impl IndexMut<usize> for OwnedIntegerSexp {
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
