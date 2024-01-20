//! # Savvy - A Simple R Interface
//!

#![doc = include_str!("../docs/design.md")]

pub mod error;
pub mod ffi;
pub mod io;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

use std::os::raw::c_char;

pub use error::{Error, Result};
pub use sexp::integer::{IntegerSexp, OwnedIntegerSexp};
pub use sexp::list::{ListSexp, OwnedListSexp};
pub use sexp::logical::{LogicalSexp, OwnedLogicalSexp};
pub use sexp::null::NullSexp;
pub use sexp::real::{OwnedRealSexp, RealSexp};
pub use sexp::string::{OwnedStringSexp, StringSexp};
pub use sexp::{Sexp, TypedSexp};

pub use sexp::external_pointer::{get_external_pointer_addr, ExternalPointerSexp, IntoExtPtrSexp};

pub use unwind_protect::unwind_protect;

// re-export
pub use savvy_macro::savvy;

use ffi::SEXP;
use savvy_ffi::{cetype_t_CE_UTF8, Rf_allocVector, Rf_mkCharLenCE};

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
                msg.as_ptr() as *const c_char,
                msg.len() as i32,
                cetype_t_CE_UTF8,
            );

            // set the error flag
            (r_error as usize | 1) as SEXP
        },
    }
}

#[macro_export]
macro_rules! r_print {
    () => {};
    ($($arg:tt)*) => { savvy::io::r_print(&format!($($arg)*), false); };
}

#[macro_export]
macro_rules! r_eprint {
    () => {};
    ($($arg:tt)*) => { savvy::io::r_eprint(&format!($($arg)*), false); };
}

#[macro_export]
macro_rules! r_println {
    () => {};
    ($($arg:tt)*) => { savvy::io::r_print(&format!($($arg)*), true); };
}

#[macro_export]
macro_rules! r_eprintln {
    () => {};
    ($($arg:tt)*) => { savvy::io::r_eprint(&format!($($arg)*), true); };
}
