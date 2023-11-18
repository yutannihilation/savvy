//! # Savvy - An Unfriendly R Interface
//!

#![doc = include_str!("../docs/design.md")]

pub mod error;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

pub use error::{Error, Result};
pub use sexp::integer::{IntegerSxp, OwnedIntegerSxp};
pub use sexp::list::{ListElement, ListSxp, OwnedListSxp};
pub use sexp::logical::{LogicalSxp, OwnedLogicalSxp};
pub use sexp::null::NullSxp;
pub use sexp::real::{OwnedRealSxp, RealSxp};
pub use sexp::string::{OwnedStringSxp, StringSxp};
pub use sexp::Sxp;

pub use sexp::external_pointer::{get_external_pointer_addr, IntoExtPtrSxp};

pub use unwind_protect::unwind_protect;

// re-export
pub use rlang_ffi_lite::SEXP;
pub use savvy_macro::savvy;

use rlang_ffi_lite::{cetype_t_CE_UTF8, REprintf, Rf_allocVector, Rf_mkCharLenCE, Rprintf};

use std::ffi::CString;

// TODO: make this r_println! macro
pub fn r_print(msg: &str) -> crate::error::Result<SEXP> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            Rprintf(msg_c_string.as_ptr());
            NullSxp.into()
        })
    }
}

pub fn r_eprint(msg: &str) -> crate::error::Result<SEXP> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            REprintf(msg_c_string.as_ptr());
            NullSxp.into()
        })
    }
}

fn alloc_vector(arg1: u32, arg2: isize) -> crate::error::Result<SEXP> {
    unsafe { unwind_protect(|| Rf_allocVector(arg1, arg2)) }
}

// This wrapper function handles Error and panicks, and flag it by setting the
// lowest bit to 1. The lowest bit is supposed to be detected (and then removed)
// on the corresponding C function.
//
// cf. https://en.wikipedia.org/wiki/Tagged_pointer
pub fn handle_error(e: crate::error::Error) -> SEXP {
    match e {
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
    }
}
