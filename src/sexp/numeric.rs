use once_cell::sync::OnceCell;

use crate::{IntegerSexp, NotAvailableValue, RealSexp, Sexp};

// --- Utils -------------------------

const I32MAX: f64 = i32::MAX as f64;
const I32MIN: f64 = i32::MIN as f64;
const TOLERANCE: f64 = 0.01; // This is super-tolerant than vctrs, but this should be sufficient.

fn try_cast_f64_to_i32(f: &f64) -> crate::Result<i32> {
    if f.is_na() || f.is_nan() {
        Ok(i32::na())
    } else if f.is_infinite() || *f > I32MAX || *f < I32MIN {
        Err(format!("{f:?} is out of range for integer").into())
    } else if (*f - f.round()).abs() > TOLERANCE {
        Err(format!("{f:?} is not integer-ish").into())
    } else {
        Ok(*f as i32)
    }
}

// --- Vector -------------------------

/// A enum to hold both the original data and the converted version. Since it
/// would be a bit confusing to expose the very implementational detail of
/// `converted` field (this is needed to return a slice), this is private.
enum PrivateNumericSexp {
    Integer {
        orig: IntegerSexp,
        converted: OnceCell<Vec<f64>>,
    },
    Real {
        orig: RealSexp,
        converted: OnceCell<Vec<i32>>,
    },
}

/// An enum to be used for `match`ing the content of `NumericSexp`.
pub enum NumericSexpVariant {
    Integer(IntegerSexp),
    Real(RealSexp),
}

/// A struct that holds either an integer or a real vector.
pub struct NumericSexp(PrivateNumericSexp);

impl NumericSexp {
    /// Return the typed SEXP.
    pub fn into_typed(self) -> NumericSexpVariant {
        match self.0 {
            PrivateNumericSexp::Integer { orig, .. } => NumericSexpVariant::Integer(orig),
            PrivateNumericSexp::Real { orig, .. } => NumericSexpVariant::Real(orig),
        }
    }

    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is real, allocates a new `Vec` and cache it. This fails when the value is
    ///
    /// - infinite
    /// - out of the range of `i32`
    /// - not integer-ish (e.g. `1.1`)
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
                    .map(try_cast_f64_to_i32)
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
    pub fn as_slice_f64(&self) -> &[f64] {
        match &self.0 {
            PrivateNumericSexp::Real { orig, .. } => orig.as_slice(),
            PrivateNumericSexp::Integer { orig, converted } => {
                if let Some(v) = converted.get() {
                    return v;
                }

                // If `converted` is not created, convert the values.
                let v_new = orig
                    .iter()
                    .map(|i| if i.is_na() { f64::na() } else { *i as f64 })
                    .collect();

                // Set v_new to converted. Otherwise, this is a temporary value and cannot be returned.
                let v = converted.get_or_init(|| v_new);

                v.as_slice()
            }
        }
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it.
    pub fn iter_i32(&self) -> crate::error::Result<std::slice::Iter<i32>> {
        self.as_slice_i32().map(|x| x.iter())
    }

    /// Returns an iterator over the underlying data of the SEXP.
    ///
    /// If the data is integer, allocates a new `Vec` and cache it.
    pub fn iter_f64(&self) -> std::slice::Iter<f64> {
        self.as_slice_f64().iter()
    }
}

impl TryFrom<Sexp> for NumericSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> Result<Self, Self::Error> {
        if !value.is_numeric() {
            let expected = "numeric";
            let actual = value.get_human_readable_type_name();
            let msg = format!("expected: {expected}\n  actual: {actual}");
            return Err(crate::Error::UnexpectedType(msg));
        }

        match value.into_typed() {
            crate::TypedSexp::Integer(i) => Ok(Self(PrivateNumericSexp::Integer {
                orig: i,
                converted: OnceCell::new(),
            })),
            crate::TypedSexp::Real(r) => Ok(Self(PrivateNumericSexp::Real {
                orig: r,
                converted: OnceCell::new(),
            })),
            _ => Err(crate::Error::GeneralError(
                "Should not reach here!".to_string(),
            )),
        }
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
            NumericScalar::Real(r) => try_cast_f64_to_i32(r),
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
}

impl TryFrom<Sexp> for NumericScalar {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> Result<Self, Self::Error> {
        if !value.is_numeric() {
            let expected = "numeric";
            let actual = value.get_human_readable_type_name();
            let msg = format!("expected: {expected}\n  actual: {actual}");
            return Err(crate::Error::UnexpectedType(msg));
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
