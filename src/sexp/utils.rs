use std::os::raw::c_char;

use savvy_ffi::{cetype_t_CE_UTF8, Rf_mkCharLenCE, SEXP};

use crate::NotAvailableValue;

pub(crate) fn assert_len(len: usize, i: usize) -> crate::error::Result<()> {
    if i >= len {
        Err(crate::error::Error::new(&format!(
            "index out of bounds: the length is {} but the index is {}",
            len, i
        )))
    } else {
        Ok(())
    }
}

pub(crate) unsafe fn str_to_charsxp(v: &str) -> crate::error::Result<SEXP> {
    unsafe {
        // We might be able to put `R_NaString` directly without using
        // <&str>::na(), but probably this is an inevitable cost of
        // providing <&str>::na().
        if v.is_na() {
            Ok(savvy_ffi::R_NaString)
        } else {
            crate::unwind_protect(|| {
                Rf_mkCharLenCE(
                    v.as_ptr() as *const c_char,
                    v.len() as i32,
                    cetype_t_CE_UTF8,
                )
            })
        }
    }
}
