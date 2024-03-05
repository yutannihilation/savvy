use savvy_ffi::{LGLSXP, LOGICAL, SET_LOGICAL_ELT, SEXP};

use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::{protect, IntegerSexp};

/// An external SEXP of a logical vector.
pub struct LogicalSexp(pub SEXP);

/// A newly-created SEXP of a logical vector.
pub struct OwnedLogicalSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(LogicalSexp);
impl_common_sexp_ops_owned!(OwnedLogicalSexp);

impl LogicalSexp {
    fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(LOGICAL(self.0), self.len()) }
    }

    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }
}

impl OwnedLogicalSexp {
    pub fn as_read_only(&self) -> LogicalSexp {
        LogicalSexp(self.inner)
    }

    fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len()) }
    }

    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    pub fn set_elt(&mut self, i: usize, v: bool) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }

        unsafe {
            SET_LOGICAL_ELT(self.inner, i as _, v as _);
        }

        Ok(())
    }

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(LGLSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(LOGICAL(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    /// Constructs a new, initialized logical vector.
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// Constructs a new, **uninitialized** logical vector.
    ///
    /// This is an expert-only version of `new()`, which can be found useful
    /// when you want to skip initialization and you are confident that the
    /// vector will be filled with values later.
    ///
    /// For example, you can use this in `TryFrom` implementation.
    ///
    /// ``` no_run
    /// struct Pair {
    ///     x: bool,
    ///     y: bool
    /// }
    ///
    /// impl TryFrom<Pair> for Sexp {
    ///     type Error = savvy::Error;
    ///
    ///     fn try_from(value: Pair) -> savvy::Result<Self> {
    ///         let mut out = unsafe { OwnedLogicalSexp::new_without_init(2)? };
    ///         out.set_elt(0, value.x)?;
    ///         out.set_elt(1, value.x)?;
    ///         
    ///         out.into()
    ///     }
    /// }
    /// ````
    ///
    /// # Safety
    ///
    /// As the memory is uninitialized, all elements must be filled values
    /// before return.
    pub unsafe fn new_without_init(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { LOGICAL(inner) };

        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }
}

impl Drop for OwnedLogicalSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to LogicalSexp ***************

impl TryFrom<Sexp> for LogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_logical() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected logicals, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<LogicalSexp> for Sexp {
    fn from(value: LogicalSexp) -> Self {
        Self(value.inner())
    }
}

impl From<LogicalSexp> for crate::error::Result<Sexp> {
    fn from(value: LogicalSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedLogicalSexp ***************

impl TryFrom<&[bool]> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[bool]) -> crate::error::Result<Self> {
        let mut out = unsafe { Self::new_without_init(value.len())? };
        for (i, v) in value.iter().enumerate() {
            out.set_elt(i, *v)?;
        }
        Ok(out)
    }
}

impl TryFrom<Vec<bool>> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<bool>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<bool> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: bool) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarLogical(value as i32))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

macro_rules! impl_try_from_rust_reals {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedLogicalSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_reals!(&[bool]);
impl_try_from_rust_reals!(Vec<bool>);
impl_try_from_rust_reals!(bool);

impl From<OwnedLogicalSexp> for Sexp {
    fn from(value: OwnedLogicalSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedLogicalSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedLogicalSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// Index for OwnedIntegerSexp ***************

pub struct LogicalSexpIter<'a> {
    iter_raw: std::slice::Iter<'a, i32>,
}

impl<'a> Iterator for LogicalSexpIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_raw.next().map(|x| *x == 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter_raw.size_hint()
    }
}

impl<'a> ExactSizeIterator for LogicalSexpIter<'a> {
    fn len(&self) -> usize {
        self.iter_raw.len()
    }
}
