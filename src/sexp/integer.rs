use std::ops::{Index, IndexMut};

use savvy_ffi::{INTEGER, INTSXP, SEXP};

use super::utils::assert_len;
use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect;
use crate::NotAvailableValue; // for na()

/// An external SEXP of an integer vector.
pub struct IntegerSexp(pub SEXP);

/// A newly-created SEXP of an integer vector
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
        self.as_slice().to_vec()
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
        self.as_slice().to_vec()
    }

    /// Set the value of the `i`-th element.
    pub fn set_elt(&mut self, i: usize, v: i32) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, v) };

        Ok(())
    }

    #[inline]
    unsafe fn set_elt_unchecked(&mut self, i: usize, v: i32) {
        unsafe { *(self.raw.add(i)) = v };
    }

    /// Set the `i`-th element to NA.
    pub fn set_na(&mut self, i: usize) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, i32::na()) };

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

    /// Constructs a new, initialized integer vector.
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// Constructs a new, **uninitialized** integer vector.
    ///
    /// This is an expert-only version of `new()`, which can be found useful
    /// when you want to skip initialization and you are confident that the
    /// vector will be filled with values later.
    ///
    /// For example, you can use this in `TryFrom` implementation.
    ///
    /// ``` no_run
    /// struct Pair {
    ///     x: i32,
    ///     y: i32
    /// }
    ///
    /// impl TryFrom<Pair> for Sexp {
    ///     type Error = savvy::Error;
    ///
    ///     fn try_from(value: Pair) -> savvy::Result<Self> {
    ///         let mut out = unsafe { OwnedIntegerSexp::new_without_init(2)? };
    ///         out[0] = value.x;
    ///         out[1] = value.y;
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
        Self::new_inner(len, false)
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

    /// Constructs a new integer vector from an iterator.
    ///
    /// Note that, if you already have a slice or vec, [`try_from_slice`] is
    /// what you want. `try_from_slice` is more performant than `try_from_iter`
    /// because it copies the underlying memory directly.
    pub fn try_from_iter<I>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = i32>,
    {
        let iter = iter.into_iter();

        match iter.size_hint() {
            (_, Some(upper)) => {
                // If the maximum length is known, use it at frist. But, the
                // iterator's length might be shorter than the reported one
                // (e.g. `(0..10).filter(|x| x % 2 == 0)`), so it needs to be
                // truncated to the actual length at last.

                let mut out = unsafe { Self::new_without_init(upper)? };

                let mut last_index = 0;
                for (i, v) in iter.enumerate() {
                    // The upper bound of size_hint() is just for optimization
                    // and what we should not trust. So, we should't use
                    // `set_elt_unchecked()` here.
                    out.set_elt(i, v)?;

                    last_index = i;
                }

                if last_index + 1 != upper {
                    unsafe {
                        savvy_ffi::SETLENGTH(out.inner, (last_index + 1) as _);
                    }
                }

                Ok(out)
            }
            (_, None) => {
                // When the length is not known at all, collect() it first.

                let v: Vec<I::Item> = iter.collect();
                v.try_into()
            }
        }
    }

    /// Constructs a new integer vector from a slice or vec.
    pub fn try_from_slice<S>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[i32]>,
    {
        let x_slice = x.as_ref();
        let mut out = unsafe { Self::new_without_init(x_slice.len())? };
        out.as_mut_slice().copy_from_slice(x_slice);
        Ok(out)
    }

    /// Constructs a new integer vector from a scalar value.
    pub fn try_from_scalar(value: i32) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarInteger(value))? };
        Self::new_from_raw_sexp(sexp, 1)
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
        value.assert_integer()?;
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
        Self::try_from_slice(value)
    }
}

impl TryFrom<Vec<i32>> for OwnedIntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<i32>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<i32> for OwnedIntegerSexp {
    type Error = crate::error::Error;

    fn try_from(value: i32) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
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
        assert_len(self.len, index).unwrap();
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedIntegerSexp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &mut *(self.raw.add(index)) }
    }
}
