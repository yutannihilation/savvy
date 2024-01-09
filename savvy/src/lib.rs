//! # Savvy - A Simple R Interface
//!

#![doc = include_str!("../docs/design.md")]

pub mod error;
pub mod ffi;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

pub use error::{Error, Result};
pub use sexp::integer::{IntegerSexp, OwnedIntegerSexp};
pub use sexp::list::{ListSexp, OwnedListSexp};
pub use sexp::logical::{LogicalSexp, OwnedLogicalSexp};
pub use sexp::null::NullSexp;
pub use sexp::real::{OwnedRealSexp, RealSexp};
pub use sexp::string::{OwnedStringSexp, StringSexp};
pub use sexp::{Sexp, TypedSexp};

pub use sexp::external_pointer::{get_external_pointer_addr, IntoExtPtrSexp};

pub use unwind_protect::unwind_protect;

// re-export
pub use savvy_macro::savvy;

use ffi::SEXP;
use savvy_ffi::{cetype_t_CE_UTF8, REprintf, Rf_allocVector, Rf_mkCharLenCE, Rprintf};

use std::ffi::CString;

// TODO: make this r_println! macro
pub fn r_print(msg: &str) -> crate::error::Result<SEXP> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            Rprintf(msg_c_string.as_ptr());
            savvy_ffi::R_NilValue
        })
    }
}

pub fn r_eprint(msg: &str) -> crate::error::Result<SEXP> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            REprintf(msg_c_string.as_ptr());
            savvy_ffi::R_NilValue
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
