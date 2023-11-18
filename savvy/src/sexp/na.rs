// NOTE: No implementation is provided for bool because R's bool is tricky.
// https://cpp11.r-lib.org/articles/cpp11.html#na

pub trait NotAvailableValue {
    fn is_na(&self) -> bool;
    fn na() -> Self;
}

impl NotAvailableValue for f64 {
    fn is_na(&self) -> bool {
        unsafe { rlang_ffi_lite::R_IsNA(*self) != 0 }
    }

    fn na() -> Self {
        unsafe { rlang_ffi_lite::R_NaReal }
    }
}

impl NotAvailableValue for i32 {
    fn is_na(&self) -> bool {
        unsafe { *self == rlang_ffi_lite::R_NaInt }
    }

    fn na() -> Self {
        unsafe { rlang_ffi_lite::R_NaInt }
    }
}

use once_cell::sync::Lazy;

pub(crate) static NA_CHAR_PTR: Lazy<&str> = Lazy::new(|| unsafe {
    let c_ptr = rlang_ffi_lite::R_CHAR(rlang_ffi_lite::R_NaString) as _;
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(c_ptr, 2))
});

impl NotAvailableValue for &str {
    fn is_na(&self) -> bool {
        self.as_ptr() == Self::na().as_ptr()
    }

    // I use the underlying "NA" string of R_NaString directry here, but this
    // wasn't possible on extendr due to some unobvious reason related to
    // concurrency.
    //
    // cf., https://github.com/extendr/extendr/issues/483#issuecomment-1435499525
    fn na() -> Self {
        NA_CHAR_PTR.as_ref()
    }
}
