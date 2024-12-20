use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use savvy_ffi::{cetype_t_CE_UTF8, Rf_mkCharLenCE, Rf_xlength, R_CHAR, SEXP};

use crate::{savvy_err, NotAvailableValue};

pub(crate) fn assert_len(len: usize, i: usize) -> crate::error::Result<()> {
    if i >= len {
        Err(savvy_err!(
            "index out of bounds: the length is {len} but the index is {i}",
        ))
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

// This doesn't handle NA.
pub(crate) unsafe fn charsxp_to_str(v: SEXP) -> &'static str {
    unsafe {
        // I bravely assume all strings are valid UTF-8 and don't use
        // `Rf_translateCharUTF8()`!
        let ptr = R_CHAR(v) as *const u8;
        let v_utf8 = std::slice::from_raw_parts(ptr, Rf_xlength(v) as usize + 1); // +1 for NUL

        // Use CStr to check the UTF-8 validity.
        CStr::from_bytes_with_nul_unchecked(v_utf8)
            .to_str()
            .unwrap_or_default()
    }
}

// Note: the result is not protected (although symbol is probably not GC-ed?)
pub(crate) fn str_to_symsxp<T: AsRef<str>>(name: T) -> crate::error::Result<Option<SEXP>> {
    let name = name.as_ref();
    if name.is_empty() {
        return Ok(None);
    }

    let name_cstr = CString::new(name)?;
    let sym = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_install(name_cstr.as_ptr())) }?;

    Ok(Some(sym))
}
