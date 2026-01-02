use savvy_ffi::SEXP;

use super::impl_common_sexp_ops;
use crate::Sexp;

/// An object SEXP (`OBJSXP`).
///
/// # Note
///
/// Historically, R's internals often refer to this as "S4", but the newer S7
/// OOP system is also built on top of `OBJSXP`. Therefore, this type can
/// represent both S4 and S7 objects.
pub struct ObjSexp(pub SEXP);

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(ObjSexp);

// Conversion into Sexp is infallible as it's just extracting the inner one.
impl From<ObjSexp> for Sexp {
    fn from(value: ObjSexp) -> Self {
        Self(value.0)
    }
}

impl From<ObjSexp> for crate::error::Result<Sexp> {
    fn from(value: ObjSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// Conversion into SEXP is infallible as it's just extracting the inner one.
impl From<ObjSexp> for SEXP {
    fn from(value: ObjSexp) -> Self {
        value.0
    }
}
