use savvy_ffi::SEXP;

use crate::Sexp;

/// This is a dummy struct solely for providing `NULL` [Result].
pub struct NullSexp;

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSexp> for Sexp {
    fn from(_value: NullSexp) -> Self {
        Self(unsafe { savvy_ffi::R_NilValue })
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSexp> for SEXP {
    fn from(value: NullSexp) -> Self {
        <Sexp>::from(value).0
    }
}

pub fn null() -> SEXP {
    unsafe { savvy_ffi::R_NilValue }
}

impl TryFrom<()> for NullSexp {
    type Error = crate::error::Error;

    fn try_from(_: ()) -> crate::error::Result<Self> {
        Ok(NullSexp)
    }
}

impl TryFrom<()> for Sexp {
    type Error = crate::error::Error;

    fn try_from(value: ()) -> crate::error::Result<Self> {
        <NullSexp>::try_from(value).map(|x| x.into())
    }
}
