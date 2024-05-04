use once_cell::sync::OnceCell;

use crate::{IntegerSexp, NotAvailableValue, RealSexp, Sexp};

pub enum NumericSexp {
    Integer {
        orig: IntegerSexp,
        converted: OnceCell<Vec<f64>>,
    },
    Real {
        orig: RealSexp,
        converted: OnceCell<Vec<i32>>,
    },
}

const I32MAX: f64 = i32::MAX as f64;
const I32MIN: f64 = i32::MIN as f64;
const TOLERANCE: f64 = 0.01; // This is super-tolerant than vctrs, but this should be sufficient.

impl NumericSexp {
    /// Extracts a slice containing the underlying data of the SEXP.
    ///
    /// If the data is real, allocates a new `Vec` and cache it. This fails when the value is
    ///
    /// - infinite
    /// - out of the range of `i32`
    /// - not integer-ish (e.g. `1.1`)
    pub fn as_slice_i32(&self) -> crate::error::Result<&[i32]> {
        match &self {
            NumericSexp::Integer { orig, .. } => Ok(orig.as_slice()),
            NumericSexp::Real { orig, converted } => {
                if let Some(v) = converted.get() {
                    return Ok(v);
                }

                // If `converted` is not created, convert the values.
                let v_new = orig
                    .iter()
                    .map(|f| {
                        if f.is_na() || f.is_nan() {
                            Ok(i32::na())
                        } else if f.is_infinite() || *f > I32MAX || *f < I32MIN {
                            Err("Out of i32 range".into())
                        } else if (*f - f.round()).abs() > TOLERANCE {
                            Err(format!("{f:?} is not integer-ish").into())
                        } else {
                            Ok(*f as i32)
                        }
                    })
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
        match &self {
            NumericSexp::Real { orig, .. } => orig.as_slice(),
            NumericSexp::Integer { orig, converted } => {
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
            crate::TypedSexp::Integer(i) => Ok(Self::Integer {
                orig: i,
                converted: OnceCell::new(),
            }),
            crate::TypedSexp::Real(r) => Ok(Self::Real {
                orig: r,
                converted: OnceCell::new(),
            }),
            _ => Err(crate::Error::GeneralError(
                "Should not reach here!".to_string(),
            )),
        }
    }
}
