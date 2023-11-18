use rlang_ffi_lite::SEXP;

/// This is a dummy struct solely for providing `NULL` [Result].
pub struct NullSxp;

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<NullSxp> for SEXP {
    fn from(_value: NullSxp) -> Self {
        unsafe { rlang_ffi_lite::R_NilValue }
    }
}
