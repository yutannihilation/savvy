pub mod error;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

pub use sexp::integer::{IntegerSxp, OwnedIntegerSxp};
pub use sexp::logical::{LogicalSxp, OwnedLogicalSxp};
pub use sexp::real::{OwnedRealSxp, RealSxp};
pub use sexp::string::{OwnedStringSxp, StringSxp};

// re-export
pub use libR_sys::SEXP;
pub use unextendr_macro::unextendr;

use protect::{
    insert_to_preserved_list, release_from_preserved_list, PreservedList, PRESERVED_LIST,
};
use unwind_protect::unwind_protect;

use libR_sys::{cetype_t_CE_UTF8, REprintf, Rf_mkCharLenCE, Rprintf};

use std::ffi::CString;

// TODO: make this r_println! macro
fn r_print(msg: String) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        Rprintf(msg_c_string.as_ptr());
    }
}

fn r_eprint(msg: String) {
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
pub fn wrapper<F>(f: F) -> SEXP
where
    F: FnOnce() -> crate::error::Result<SEXP>,
{
    match f() {
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
