use savvy_ffi::{LOGICAL_ELT, RAW_ELT};

use crate::{IntegerSexp, LogicalSexp, RawSexp, RealSexp, Sexp, StringSexp};

use super::na::NotAvailableValue;

macro_rules! impl_try_from_scalar {
    ($scalar_ty: ty, $sexp_ty: ty) => {
        impl TryFrom<Sexp> for $scalar_ty {
            type Error = crate::error::Error;

            fn try_from(value: Sexp) -> crate::error::Result<Self> {
                let value = <$sexp_ty>::try_from(value)?;
                if value.len() != 1 {
                    return Err(crate::error::Error::NotScalar);
                }

                let result = value.iter().next().unwrap();

                if result.is_na() {
                    return Err(crate::error::Error::NotScalar);
                }

                Ok(result.clone())
            }
        }
    };
}

impl_try_from_scalar!(i32, IntegerSexp);
impl_try_from_scalar!(f64, RealSexp);
impl_try_from_scalar!(&str, StringSexp);

// bool doesn't have na() method, so define manually.
impl TryFrom<Sexp> for bool {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        let value = <LogicalSexp>::try_from(value)?;
        if value.len() != 1 {
            return Err(crate::error::Error::NotScalar);
        }

        let result_int = unsafe { LOGICAL_ELT(value.0, 0) };
        if result_int.is_na() {
            return Err(crate::error::Error::NotScalar);
        }

        Ok(result_int == 1)
    }
}

// raw doesn't have na() method, so define manually.
impl TryFrom<Sexp> for u8 {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        let value = <RawSexp>::try_from(value)?;
        if value.len() != 1 {
            return Err(crate::error::Error::NotScalar);
        }

        Ok(unsafe { RAW_ELT(value.0, 0) })
    }
}

impl TryFrom<Sexp> for usize {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        let value = <i32>::try_from(value)?;
        <Self>::try_from(value).map_err(|e| crate::Error::new(&e.to_string()))
    }
}
