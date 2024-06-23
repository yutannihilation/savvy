use std::ops::{Index, IndexMut};

use savvy_ffi::{RAW, RAWSXP, SEXP};

use super::utils::assert_len;
use super::{impl_common_sexp_ops, impl_common_sexp_ops_owned, Sexp};
use crate::protect::{self, local_protect};

/// An external SEXP of a raw vector.
pub struct RawSexp(pub SEXP);

/// A newly-created SEXP of a raw vector.
pub struct OwnedRawSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut u8,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(RawSexp);
impl_common_sexp_ops_owned!(OwnedRawSexp);

impl RawSexp {
    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// # let raw_sexp = savvy::OwnedRawSexp::try_from_slice([1_u8, 2, 3])?.as_read_only();
    /// // `raw_sexp` is c(1L, 2L, 3L)
    /// assert_eq!(raw_sexp.as_slice(), &[1, 2, 3]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(RAW(self.inner()) as _, self.len()) }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// # let raw_sexp = savvy::OwnedRawSexp::try_from_slice([1_u8, 2, 3])?.as_read_only();
    /// // `raw_sexp` is c(1L, 2L, 3L)
    /// let mut iter = raw_sexp.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.as_slice(), &[2, 3]);
    /// ```
    ///
    /// # Technical Note
    ///
    /// If the input is an ALTREP, this materialize it first, so it might not be
    /// most efficient. However, it seems Rust's slice implementation is very
    /// fast, so probably being efficient for ALTREP is not worth giving up the
    /// benefit.
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.as_slice().iter()
    }

    /// Copies the underlying data of the SEXP into a new `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// # let raw_sexp = savvy::OwnedRawSexp::try_from_slice([1_u8, 2, 3])?.as_read_only();
    /// // `raw_sexp` is c(1L, 2L, 3L)
    /// assert_eq!(raw_sexp.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl OwnedRawSexp {
    /// Returns the read-only version of the wrapper. This is mainly for testing
    /// purposes.
    pub fn as_read_only(&self) -> RawSexp {
        RawSexp(self.inner)
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let raw_sexp = OwnedRawSexp::try_from_slice([1_u8, 2, 3])?;
    /// assert_eq!(raw_sexp.as_slice(), &[1, 2, 3]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len) }
    }

    /// Extracts a mutable slice containing the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let mut raw_sexp = OwnedRawSexp::new(3)?;
    /// let s = raw_sexp.as_mut_slice();
    /// s[2] = 10;
    /// assert_eq!(raw_sexp.as_slice(), &[0, 0, 10]);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.raw, self.len) }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the underlying data of the SEXP.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let mut raw_sexp = OwnedRawSexp::try_from_slice([1_u8, 2, 3])?;
    /// raw_sexp.iter_mut().for_each(|x| *x = *x * 2);
    /// assert_eq!(raw_sexp.as_slice(), &[2, 4, 6]);
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<u8> {
        self.as_mut_slice().iter_mut()
    }

    /// Copies the underlying data of the SEXP into a new `Vec`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    /// Set the value of the `i`-th element. `i` starts from `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let mut raw_sexp = OwnedRawSexp::new(3)?;
    /// raw_sexp.set_elt(2, 10)?;
    /// assert_eq!(raw_sexp.as_slice(), &[0, 0, 10]);
    /// ```
    pub fn set_elt(&mut self, i: usize, v: u8) -> crate::error::Result<()> {
        super::utils::assert_len(self.len, i)?;

        unsafe { self.set_elt_unchecked(i, v) };

        Ok(())
    }

    #[inline]
    unsafe fn set_elt_unchecked(&mut self, i: usize, v: u8) {
        unsafe { *(self.raw.add(i)) = v };
    }

    fn new_inner(len: usize, init: bool) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(RAWSXP, len as _)?;

        // Fill the vector with default values
        if init {
            unsafe {
                std::ptr::write_bytes(RAW(inner), 0, len);
            }
        }

        Self::new_from_raw_sexp(inner, len)
    }

    /// Constructs a new, initialized raw vector.
    ///
    /// ```
    /// let x = savvy::OwnedRawSexp::new(3)?;
    /// assert_eq!(x.as_slice(), &[0, 0, 0]);
    /// ```
    pub fn new(len: usize) -> crate::error::Result<Self> {
        Self::new_inner(len, true)
    }

    /// Constructs a new, **uninitialized** raw vector.
    ///
    /// This is an expert-only version of `new()`, which can be found useful
    /// when you want to skip initialization and you are confident that the
    /// vector will be filled with values later.
    ///
    /// For example, you can use this in `TryFrom` implementation.
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// struct Pair {
    ///     x: u8,
    ///     y: u8
    /// }
    ///
    /// impl TryFrom<Pair> for OwnedRawSexp {
    ///     type Error = savvy::Error;
    ///
    ///     fn try_from(value: Pair) -> savvy::Result<Self> {
    ///         let mut out = unsafe { OwnedRawSexp::new_without_init(2)? };
    ///         out[0] = value.x;
    ///         out[1] = value.y;
    ///         
    ///         Ok(out)
    ///     }
    /// }
    ///
    /// let pair = Pair { x: 1, y: 2 };
    /// let raw_sexp = <OwnedRawSexp>::try_from(pair)?;
    /// assert_eq!(raw_sexp.as_slice(), &[1, 2]);
    /// ```
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
        let raw = unsafe { RAW(inner) };

        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }

    /// Constructs a new complex vector from an iterator.
    ///
    /// Note that, if you already have a slice or vec, [`try_from_slice()`][1]
    /// is what you want. `try_from_slice` is more performant than
    /// `try_from_iter` because it copies the underlying memory directly.
    ///
    /// [1]: `Self::try_from_slice()`
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let iter = (0..10).filter(|x| x % 2 == 0);
    /// let raw_sexp = OwnedRawSexp::try_from_iter(iter)?;
    /// assert_eq!(raw_sexp.as_slice(), &[0, 2, 4, 6, 8]);
    /// ```
    pub fn try_from_iter<I>(iter: I) -> crate::error::Result<Self>
    where
        I: IntoIterator<Item = u8>,
    {
        let iter = iter.into_iter();

        match iter.size_hint() {
            (_, Some(upper)) => {
                // If the maximum length is known, use it at frist. But, the
                // iterator's length might be shorter than the reported one
                // (e.g. `(0..10).filter(|x| x % 2 == 0)`), so it needs to be
                // truncated to the actual length at last.

                let inner = crate::alloc_vector(RAWSXP, upper as _)?;
                local_protect(inner);
                let raw = unsafe { RAW(inner) };

                let mut last_index = 0;
                for (i, v) in iter.enumerate() {
                    // The upper bound of size_hint() is just for optimization
                    // and what we should not trust.
                    assert_len(upper, i)?;
                    unsafe { *(raw.add(i)) = v };

                    last_index = i;
                }

                let new_len = last_index + 1;
                if new_len == upper {
                    // If the length is the same as expected, use it as it is.
                    Self::new_from_raw_sexp(inner, upper)
                } else {
                    // If the length is shorter than expected, re-allocate a new
                    // SEXP and copy the values into it.
                    let out = unsafe { Self::new_without_init(new_len)? };
                    let dst = unsafe { std::slice::from_raw_parts_mut(out.raw, new_len) };
                    // `raw` is longer than new_len, but the elements over new_len are ignored
                    let src = unsafe { std::slice::from_raw_parts(raw, new_len) };
                    dst.copy_from_slice(src);

                    Ok(out)
                }
            }
            (_, None) => {
                // When the length is not known at all, collect() it first.

                let v: Vec<I::Item> = iter.collect();
                v.try_into()
            }
        }
    }

    /// Constructs a new raw vector from a slice or vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let raw_sexp = OwnedRawSexp::try_from_slice([1_u8, 2, 3])?;
    /// assert_eq!(raw_sexp.as_slice(), &[1, 2, 3]);
    /// ```
    pub fn try_from_slice<S>(x: S) -> crate::error::Result<Self>
    where
        S: AsRef<[u8]>,
    {
        let x_slice = x.as_ref();
        let mut out = unsafe { Self::new_without_init(x_slice.len())? };
        out.as_mut_slice().copy_from_slice(x_slice);
        Ok(out)
    }

    /// Constructs a new raw vector from a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::OwnedRawSexp;
    ///
    /// let raw_sexp = OwnedRawSexp::try_from_scalar(1)?;
    /// assert_eq!(raw_sexp.as_slice(), &[1]);
    /// ```
    pub fn try_from_scalar(value: u8) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarRaw(value))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

impl Drop for OwnedRawSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

// conversions from/to RawSexp ***************

impl TryFrom<Sexp> for RawSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_raw()?;
        Ok(Self(value.0))
    }
}

impl From<RawSexp> for Sexp {
    fn from(value: RawSexp) -> Self {
        Self(value.inner())
    }
}

impl From<RawSexp> for crate::error::Result<Sexp> {
    fn from(value: RawSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedRawSexp ***************

impl TryFrom<&[u8]> for OwnedRawSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[u8]) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<Vec<u8>> for OwnedRawSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<u8>) -> crate::error::Result<Self> {
        Self::try_from_slice(value)
    }
}

impl TryFrom<u8> for OwnedRawSexp {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        Self::try_from_scalar(value)
    }
}

impl From<OwnedRawSexp> for Sexp {
    fn from(value: OwnedRawSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedRawSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedRawSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

macro_rules! impl_try_from_rust_raws {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedRawSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_raws!(&[u8]);
impl_try_from_rust_raws!(Vec<u8>);
impl_try_from_rust_raws!(u8);

// Index for OwnedRawSexp ***************

impl Index<usize> for OwnedRawSexp {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &*(self.raw.add(index)) }
    }
}

impl IndexMut<usize> for OwnedRawSexp {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert_len(self.len, index).unwrap();
        unsafe { &mut *(self.raw.add(index)) }
    }
}

#[cfg(feature = "savvy-test")]
mod test {
    use super::OwnedRawSexp;
    use crate::NotAvailableValue;

    #[test]
    fn test_raw() -> crate::Result<()> {
        let mut x = OwnedRawSexp::new(3)?;
        assert_eq!(x.as_slice(), &[0, 0, 0]);

        // set_elt()
        x.set_elt(0, 1)?;
        assert_eq!(x.as_slice(), &[1, 0, 0]);

        // IndexMut
        x[1] = 2;
        assert_eq!(x.as_slice(), &[1, 2, 0]);

        Ok(())
    }
}
