use std::sync::OnceLock;

use crate::{savvy_err, IntegerSexp, NotAvailableValue, RealSexp, Sexp};

// --- Utils -------------------------

const I32MAX: f64 = i32::MAX as f64;
const I32MIN: f64 = i32::MIN as f64;

// f64 can represent 2^53
//
// cf. https://en.wikipedia.org/wiki/Double-precision_floating-point_format,
//     https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER
#[cfg(target_pointer_width = "64")]
const F64_MAX_CASTABLE_TO_USIZE: f64 = (2_u64.pow(53) - 1) as f64;

// On 32-bit target, usize::MAX is less than 2^53
#[cfg(target_pointer_width = "32")]
const F64_MAX_CASTABLE_TO_USIZE: f64 = usize::MAX as f64;

const TOLERANCE: f64 = 0.01; // This is super-tolerant than vctrs, but this should be sufficient.

fn try_cast_f64_to_i32(f: f64) -> crate::Result<i32> {
    if f.is_na() || f.is_nan() {
        Ok(i32::na())
    } else if f.is_infinite() || !(I32MIN..=I32MAX).contains(&f) {
        Err(savvy_err!("{f:?} is out of range for integer"))
    } else if (f - f.round()).abs() > TOLERANCE {
        Err(savvy_err!("{f:?} is not integer-ish"))
    } else {
        Ok(f as i32)
    }
}

fn cast_i32_to_f64(i: i32) -> f64 {
    if i.is_na() {
        f64::na()
    } else {
        i as f64
    }
}

fn try_cast_i32_to_usize(i: i32) -> crate::error::Result<usize> {
    if i.is_na() {
        Err(savvy_err!("cannot convert NA to usize"))
    } else {
        Ok(<usize>::try_from(i)?)
    }
}

fn try_cast_f64_to_usize(f: f64) -> crate::Result<usize> {
    if f.is_na() || f.is_nan() {
        Err(savvy_err!("cannot convert NA or NaN to usize"))
    } else if f.is_infinite() || !(0f64..=F64_MAX_CASTABLE_TO_USIZE).contains(&f) {
        Err(savvy_err!(
            "{f:?} is out of range that can be safely converted to usize"
        ))
    } else if (f - f.round()).abs() > TOLERANCE {
        Err(savvy_err!("{f:?} is not integer-ish"))
    } else {
        Ok(f as usize)
    }
}

// --- Vector -------------------------

/// A enum to hold both the original data and the converted version. Since it
/// would be a bit confusing to expose the very implementational detail of
/// `converted` field (this is needed to return a slice), this is private.
enum PrivateNumericSexp {
    Integer {
        orig: IntegerSexp,
        converted: OnceLock<Vec<f64>>,
    },
    Real {
        orig: RealSexp,
        converted: OnceLock<Vec<i32>>,
    },
}

/// An enum to be used for `match`ing the content of `NumericSexp`.
pub enum NumericTypedSexp {
    Integer(IntegerSexp),
    Real(RealSexp),
}

/// A struct that holds either an integer or a real vector.
pub struct NumericSexp(PrivateNumericSexp);

impl NumericSexp {
    #[inline]
    fn inner(&self) -> savvy_ffi::SEXP {
        match &self.0 {
            PrivateNumericSexp::Integer { orig, .. } => orig.0,
            PrivateNumericSexp::Real { orig, .. } => orig.0,
        }
    }

    /// Returns the reference to the raw SEXP. This is convenient when
    /// the lifetime is needed (e.g. returning a slice).
    #[inline]
    pub(crate) fn inner_ref(&self) -> &savvy_ffi::SEXP {
        match &self.0 {
            PrivateNumericSexp::Integer { orig, .. } => &orig.0,
            PrivateNumericSexp::Real { orig, .. } => &orig.0,
        }
    }

    /// Returns the length of the SEXP.
    pub fn len(&self) -> usize {
        unsafe { savvy_ffi::Rf_xlength(self.inner()) as _ }
    }

    /// Returns `true` if the SEXP is of zero-length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the specified attribute.
    pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
        crate::Sexp(self.inner()).get_attrib(attr)
    }

    /// Returns the names.
    pub fn get_names(&self) -> Option<Vec<&'static str>> {
        crate::Sexp(self.inner()).get_names()
    }

    /// Returns the S3 class.
    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        crate::Sexp(self.inner()).get_class()
    }

    /// Returns the dimension.
    pub fn get_dim(&self) -> Option<&[i32]> {
        // In order to maintain the lifetime, this cannot rely on the
        // Sexp's method. Otherwise, you'll see the "cannot return
        // reference to temporary value" error.
        unsafe { crate::sexp::get_dim_from_sexp(self.inner_ref()) }
    }

    /// Returns the typed SEXP.
    pub fn into_typed(self) -> NumericTypedSexp {
        match self.0 {
            PrivateNumericSexp::Integer { orig, .. } => NumericTypedSexp::Integer(orig),
            PrivateNumericSexp::Real { orig, .. } => NumericTypedSexp::Real(orig),
        }
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is real, allocates a new `Vec` and cache it. This fails when
    /// the value is
    ///
    /// - infinite
    /// - out of the range of `i32`
    /// - not integer-ish (e.g. `1.1`)
    ///
    /// # Examples
    ///
    /// ```
    /// # let int_sexp = savvy::OwnedRealSexp::try_from_slice([1.0, 2.0, 3.0])?.as_read_only();
    /// # let num_sexp: savvy::NumericSexp = int_sexp.try_into()?;
    /// // `num_sexp` is c(1, 2, 3)
    /// assert_eq!(num_sexp.as_slice_i32().unwrap(), &[1, 2, 3]);
    /// ```
    pub fn as_slice_i32(&self) -> crate::error::Result<&[i32]> {
        match &self.0 {
            PrivateNumericSexp::Integer { orig, .. } => Ok(orig.as_slice()),
            PrivateNumericSexp::Real { orig, converted } => {
                if let Some(v) = converted.get() {
                    return Ok(v);
                }

                // If `converted` is not created, convert the values.
                let v_new = orig
                    .iter()
                    .map(|x| try_cast_f64_to_i32(*x))
                    .collect::<crate::Result<Vec<i32>>>()?;

                // Set v_new to converted. Otherwise, this is a temporary value and cannot be returned.
                let v = converted.get_or_init(|| v_new);

                Ok(v.as_slice())
            }
        }
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it.
    ///
    /// # Examples
    ///
    /// ```
    /// # let int_sexp = savvy::OwnedIntegerSexp::try_from_slice([1, 2, 3])?.as_read_only();
    /// # let num_sexp: savvy::NumericSexp = int_sexp.try_into()?;
    /// // `num_sexp` is c(1L, 2L, 3L)
    /// assert_eq!(num_sexp.as_slice_f64(), &[1.0, 2.0, 3.0]);
    /// ```
    pub fn as_slice_f64(&self) -> &[f64] {
        match &self.0 {
            PrivateNumericSexp::Real { orig, .. } => orig.as_slice(),
            PrivateNumericSexp::Integer { orig, converted } => {
                if let Some(v) = converted.get() {
                    return v;
                }

                // If `converted` is not created, convert the values.
                let v_new = orig.iter().map(|i| cast_i32_to_f64(*i)).collect();

                // Set v_new to converted. Otherwise, this is a temporary value and cannot be returned.
                let v = converted.get_or_init(|| v_new);

                v.as_slice()
            }
        }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it. While this
    /// method itself doesn't fail, the iterator might fail to return value in
    /// case the conversion failed, i.e. when the value is
    ///
    /// - infinite
    /// - out of the range of `i32`
    /// - not integer-ish (e.g. `1.1`)
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::NotAvailableValue;
    ///
    /// # let int_sexp = savvy::OwnedIntegerSexp::try_from_slice([1, i32::na()])?.as_read_only();
    /// # let num_sexp: savvy::NumericSexp = int_sexp.try_into()?;
    /// // `num_sexp` is c(1, NA)
    /// let mut iter = num_sexp.iter_f64();
    ///
    /// assert_eq!(iter.next(), Some(1.0));
    ///
    /// // NA is propagated
    /// let e1 = iter.next();
    /// assert!(e1.is_some());
    /// assert!(e1.unwrap().is_na());
    /// ```
    pub fn iter_i32(&self) -> NumericIteratorI32 {
        match &self.0 {
            PrivateNumericSexp::Integer { orig, .. } => NumericIteratorI32 {
                sexp: self,
                raw: Some(orig.as_slice()),
                i: 0,
                len: self.len(),
            },
            PrivateNumericSexp::Real { converted, .. } => {
                let raw = converted.get().map(|x| x.as_slice());
                NumericIteratorI32 {
                    sexp: self,
                    raw,
                    i: 0,
                    len: self.len(),
                }
            }
        }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use savvy::NotAvailableValue;
    ///
    /// # let int_sexp = savvy::OwnedRealSexp::try_from_slice([1.0, f64::na(), 1.1])?.as_read_only();
    /// # let num_sexp: savvy::NumericSexp = int_sexp.try_into()?;
    /// // `num_sexp` is c(1.0, NA, 1.1)
    /// let mut iter = num_sexp.iter_i32();
    ///
    /// let e0 = iter.next();
    /// assert!(e0.is_some());
    /// assert_eq!(e0.unwrap()?, 1);
    ///
    /// // NA is propagated
    /// let e1 = iter.next();
    /// assert!(e1.is_some());
    /// assert!(e1.unwrap()?.is_na());
    ///
    /// // 1.1 is not integer-ish, so the conversion fails.
    /// let e2 = iter.next();
    /// assert!(e2.is_some());
    /// assert!(e2.unwrap().is_err());
    /// ```
    pub fn iter_f64(&self) -> NumericIteratorF64 {
        match &self.0 {
            PrivateNumericSexp::Real { orig, .. } => NumericIteratorF64 {
                sexp: self,
                raw: Some(orig.as_slice()),
                i: 0,
                len: self.len(),
            },
            PrivateNumericSexp::Integer { converted, .. } => {
                let raw = converted.get().map(|x| x.as_slice());
                NumericIteratorF64 {
                    sexp: self,
                    raw,
                    i: 0,
                    len: self.len(),
                }
            }
        }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    pub fn iter_usize(&self) -> NumericIteratorUsize {
        NumericIteratorUsize {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    // Note: If the conversion is needed, to_vec_*() would copy the values twice
    // because it creates a `Vec` from to_slice(). This is inefficient, but I'm
    // not sure which is worse to always creates a `Vec` from scratch or use the
    // cached one. So, I chose not to implement the method.
}

impl TryFrom<Sexp> for NumericSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> Result<Self, Self::Error> {
        if !value.is_numeric() {
            let expected = "numeric".to_string();
            let actual = value.get_human_readable_type_name().to_string();
            return Err(crate::error::Error::UnexpectedType { expected, actual });
        }

        match value.into_typed() {
            crate::TypedSexp::Integer(i) => Ok(Self(PrivateNumericSexp::Integer {
                orig: i,
                converted: OnceLock::new(),
            })),
            crate::TypedSexp::Real(r) => Ok(Self(PrivateNumericSexp::Real {
                orig: r,
                converted: OnceLock::new(),
            })),
            _ => Err(crate::Error::GeneralError(
                "Should not reach here!".to_string(),
            )),
        }
    }
}

impl TryFrom<IntegerSexp> for NumericSexp {
    type Error = crate::error::Error;

    fn try_from(value: IntegerSexp) -> Result<Self, Self::Error> {
        Ok(Self(PrivateNumericSexp::Integer {
            orig: value,
            converted: OnceLock::new(),
        }))
    }
}

impl TryFrom<RealSexp> for NumericSexp {
    type Error = crate::error::Error;

    fn try_from(value: RealSexp) -> Result<Self, Self::Error> {
        Ok(Self(PrivateNumericSexp::Real {
            orig: value,
            converted: OnceLock::new(),
        }))
    }
}

// --- Iterator -----------------------

/// An iterator that returns `i32` wrapped with `Result`.
///
/// - If the underlying data is integer, use the value as it is.
/// - If the underlying data is real, but there's already the `i32` values
///   converted from the real, use the values.
/// - Otherwise, convert a real value to `i32` on the fly. This is fallible.
pub struct NumericIteratorI32<'a> {
    sexp: &'a NumericSexp,
    raw: Option<&'a [i32]>,
    i: usize,
    len: usize,
}

impl Iterator for NumericIteratorI32<'_> {
    type Item = crate::error::Result<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        match &self.raw {
            Some(x) => Some(Ok(x[i])),
            None => {
                if let PrivateNumericSexp::Real { orig, .. } = &self.sexp.0 {
                    Some(try_cast_f64_to_i32(orig.as_slice()[i]))
                } else {
                    unreachable!("Integer must have the raw slice.");
                }
            }
        }
    }
}

/// An iterator that returns `f64`.
///
/// - If the underlying data is real, use the value as it is.
/// - If the underlying data is integer, but there's already the `f64` values
///   converted from the integer, use the values.
/// - Otherwise, convert an integer value to `f64` on the fly.
pub struct NumericIteratorF64<'a> {
    sexp: &'a NumericSexp,
    raw: Option<&'a [f64]>,
    i: usize,
    len: usize,
}

impl Iterator for NumericIteratorF64<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        match &self.raw {
            Some(x) => Some(x[i]),
            None => {
                if let PrivateNumericSexp::Integer { orig, .. } = &self.sexp.0 {
                    Some(cast_i32_to_f64(orig.as_slice()[i]))
                } else {
                    unreachable!("Real must have the raw slice.");
                }
            }
        }
    }
}

/// An iterator that returns `usize` wrapped with `Result`.
pub struct NumericIteratorUsize<'a> {
    sexp: &'a NumericSexp,
    i: usize,
    len: usize,
}

impl Iterator for NumericIteratorUsize<'_> {
    type Item = crate::error::Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        let elem = match &self.sexp.0 {
            PrivateNumericSexp::Integer { orig, .. } => try_cast_i32_to_usize(orig.as_slice()[i]),
            PrivateNumericSexp::Real { orig, .. } => try_cast_f64_to_usize(orig.as_slice()[i]),
        };

        Some(elem)
    }
}

// --- Scalar -------------------------

/// A struct that holds either an integer or a real scalar.
pub enum NumericScalar {
    Integer(i32),
    Real(f64),
}

impl NumericScalar {
    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is real, allocates a new `Vec` and cache it. This fails when the value is
    ///
    /// - infinite
    /// - out of the range of `i32`
    /// - not integer-ish (e.g. `1.1`)
    pub fn as_i32(&self) -> crate::error::Result<i32> {
        match &self {
            NumericScalar::Integer(i) => Ok(*i),
            NumericScalar::Real(r) => try_cast_f64_to_i32(*r),
        }
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it.
    pub fn as_f64(&self) -> f64 {
        match &self {
            NumericScalar::Integer(i) => *i as f64,
            NumericScalar::Real(r) => *r,
        }
    }

    pub fn as_usize(&self) -> crate::error::Result<usize> {
        match &self {
            NumericScalar::Integer(i) => try_cast_i32_to_usize(*i),
            NumericScalar::Real(r) => try_cast_f64_to_usize(*r),
        }
    }
}

impl TryFrom<Sexp> for NumericScalar {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> Result<Self, Self::Error> {
        if !value.is_numeric() {
            let expected = "numeric".to_string();
            let actual = value.get_human_readable_type_name().to_string();
            return Err(crate::error::Error::UnexpectedType { expected, actual });
        }

        match value.into_typed() {
            crate::TypedSexp::Integer(i) => {
                if i.len() != 1 {
                    return Err(crate::error::Error::NotScalar);
                }

                let i_scalar = *i.iter().next().unwrap();

                if i_scalar.is_na() {
                    return Err(crate::error::Error::NotScalar);
                }

                Ok(Self::Integer(i_scalar))
            }
            crate::TypedSexp::Real(r) => {
                if r.len() != 1 {
                    return Err(crate::error::Error::NotScalar);
                }

                let r_scalar = *r.iter().next().unwrap();

                if r_scalar.is_na() {
                    return Err(crate::error::Error::NotScalar);
                }

                Ok(Self::Real(r_scalar))
            }

            _ => Err(crate::Error::GeneralError(
                "Should not reach here!".to_string(),
            )),
        }
    }
}
