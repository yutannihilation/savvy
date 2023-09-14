use crate::{IntegerSxp, LogicalSxp, RealSxp, StringSxp, Sxp};

use super::na::NotAvailableValue;

macro_rules! impl_try_from_scalar {
    ($scalar_ty: ty, $sexp_ty: ty) => {
        impl TryFrom<Sxp> for $scalar_ty {
            type Error = crate::error::Error;

            fn try_from(value: Sxp) -> crate::error::Result<Self> {
                let value = <$sexp_ty>::try_from(value)?;
                if value.len() != 1 {
                    return Err(crate::error::Error::NotScalar);
                }

                // Note: use iter().next() instead of elt(), because StringSxp::elt() returns SEXP.
                let result = value.iter().next().unwrap();

                if result.is_na() {
                    return Err(crate::error::Error::NotScalar);
                }

                Ok(result)
            }
        }
    };
}

impl_try_from_scalar!(i32, IntegerSxp);
impl_try_from_scalar!(f64, RealSxp);
impl_try_from_scalar!(&str, StringSxp);

// bool doesn't have na() method, so define manually.
impl TryFrom<Sxp> for bool {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        let value = <LogicalSxp>::try_from(value)?;
        if value.len() != 1 {
            return Err(crate::error::Error::NotScalar);
        }

        // Note: use iter().next() instead of elt(), because StringSxp::elt() returns SEXP.
        let result = value.iter().next().unwrap();

        Ok(result)
    }
}
