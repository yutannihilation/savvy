use savvy_ffi::SEXP;

use crate::Sxp;

/// This is a dummy struct solely for providing `NULL` [Result].
pub struct NullSxp;

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSxp> for Sxp {
    fn from(_value: NullSxp) -> Self {
        Self(unsafe { savvy_ffi::R_NilValue })
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSxp> for SEXP {
    fn from(value: NullSxp) -> Self {
        <Sxp>::from(value).0
    }
}

pub fn null() -> SEXP {
    unsafe { savvy_ffi::R_NilValue }
}
