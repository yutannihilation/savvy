use libR_sys::SEXP;

/// This is a dummy struct solely for providing `NULL` [Result].
pub struct NullSxp;

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSxp> for crate::error::Result<SEXP> {
    fn from(_value: NullSxp) -> Self {
        Ok(unsafe { libR_sys::R_NilValue })
    }
}
