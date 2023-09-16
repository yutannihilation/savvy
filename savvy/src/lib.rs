pub mod error;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

pub use error::{Error, Result};
pub use sexp::integer::{IntegerSxp, OwnedIntegerSxp};
pub use sexp::list::{ListSxp, OwnedListSxp};
pub use sexp::logical::{LogicalSxp, OwnedLogicalSxp};
pub use sexp::null::NullSxp;
pub use sexp::real::{OwnedRealSxp, RealSxp};
pub use sexp::string::{OwnedStringSxp, StringSxp};
pub use sexp::Sxp;

pub use sexp::external_pointer::{get_external_pointer_addr, IntoExtPtrSxp};

// re-export
pub use libR_sys::SEXP;
pub use savvy_macro::savvy;

use libR_sys::{cetype_t_CE_UTF8, REprintf, Rf_mkCharLenCE, Rprintf};

use std::ffi::CString;

// TODO: make this r_println! macro
pub fn r_print(msg: &str) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        Rprintf(msg_c_string.as_ptr());
    }
}

pub fn r_eprint(msg: &str) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        REprintf(msg_c_string.as_ptr());
    }
}

// This wrapper function handles Error and panicks, and flag it by setting the
// lowest bit to 1. The lowest bit is supposed to be detected (and then removed)
// on the corresponding C function.
//
// cf. https://en.wikipedia.org/wiki/Tagged_pointer
pub fn handle_result(result: crate::error::Result<SEXP>) -> SEXP {
    match result {
        // NOTE: At first, I wrote `(res as usize & !1) as SEXP` to ensure the
        // error flag is off, but it's unnecessary because an SEXP should be an
        // aligned address, otherwise it should have failed before this point,
        // and unaligned address cannot be restored on the C function's side
        // anyway.
        Ok(res) => res,

        Err(e) => match e {
            // The token is already tagged, so pass it as it is.
            error::Error::Aborted(token) => token,

            // In other cases, return the error string with the tag
            e => unsafe {
                let msg = e.to_string();
                let r_error = Rf_mkCharLenCE(
                    msg.as_ptr() as *const i8,
                    msg.len() as i32,
                    cetype_t_CE_UTF8,
                );

                // set the error flag
                (r_error as usize | 1) as SEXP
            },
        },
    }
}
