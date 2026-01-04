// TODO: Remove this when edition is set to 2024
#![warn(unsafe_op_in_unsafe_fn)]

//! # Savvy - A Simple R Interface
//!
//! **savvy** is a simple R extension interface using Rust, like the [extendr]
//! framework. The name "savvy" comes from the Japanese word "錆" (pronounced as
//! `sàbí`), which means "Rust".
//!
//! With savvy, you can automatically generate R functions from Rust code.
//! Please refer to [the user guide] for a detailed introduction!
//!
//! [extendr]: https://extendr.github.io/
//! [the user guide]: https://yutannihilation.github.io/savvy/guide/
//!
//! ## Example
//!
//! **Rust**:
//!
//! ```no_run
//! use savvy::savvy;
//!
//! /// Convert to Upper-case
//! ///
//! /// @param x A character vector.
//! /// @export
//! #[savvy]
//! fn to_upper(x: StringSexp) -> savvy::Result<savvy::Sexp> {
//!     // Use `Owned{type}Sexp` to allocate an R vector for output.
//!     let mut out = OwnedStringSexp::new(x.len())?;
//!
//!     for (i, e) in x.iter().enumerate() {
//!         // To Rust, missing value is an ordinary value. In `&str`'s case, it's just "NA".
//!         // You have to use `.is_na()` method to distinguish the missing value.
//!         if e.is_na() {
//!             // Set the i-th element to NA
//!             out.set_na(i)?;
//!             continue;
//!         }
//!
//!         let e_upper = e.to_uppercase();
//!         out.set_elt(i, e_upper.as_str())?;
//!     }
//!
//!     out.into()
//! }
//! ```
//!
//! **R**:
//!
//! ```text
//! to_upper(c("a", "b", "c"))
//! #> [1] "A" "B" "C"
//! ```
//!
//! ## Feature flags
//!
//! * **complex**: Provides the support for complex with the [num-complex]
//!   crate.
//!
//! * **altrep**: Provides the support for ALTREP.
//!
//! * **logger**: Provides an env_logger for R's stderr.
//!
//! * **use-custom-error**: Opt-out the auto error conversion in order to allow
//!   defining custom errors. See [Error handling] section of the user guide.
//!
//! [num-complex]: https://crates.io/crates/num-complex  
//! [Error handling]: https://yutannihilation.github.io/savvy/guide/error.html

pub mod error;
pub mod eval;
pub mod ffi;
pub mod io;
pub mod panic_hook;
pub mod protect;
pub mod sexp;
pub mod unwind_protect;

#[cfg(feature = "altrep")]
pub mod altrep;

pub mod log;

use std::os::raw::c_char;

pub use error::{Error, Result};
pub use sexp::environment::EnvironmentSexp;
pub use sexp::external_pointer::{
    get_external_pointer_addr, take_external_pointer_value, ExternalPointerSexp, IntoExtPtrSexp,
};
pub use sexp::function::{FunctionArgs, FunctionSexp};
pub use sexp::integer::{IntegerSexp, OwnedIntegerSexp};
pub use sexp::list::{ListSexp, OwnedListSexp};
pub use sexp::logical::{LogicalSexp, OwnedLogicalSexp};
pub use sexp::na::NotAvailableValue;
pub use sexp::null::NullSexp;
pub use sexp::numeric::{NumericScalar, NumericSexp, NumericTypedSexp};
pub use sexp::obj::ObjSexp;
pub use sexp::raw::{OwnedRawSexp, RawSexp};
pub use sexp::real::{OwnedRealSexp, RealSexp};
pub use sexp::string::{OwnedStringSexp, StringSexp};
pub use sexp::{Sexp, TypedSexp};

#[cfg(feature = "complex")]
pub use sexp::complex::{ComplexSexp, OwnedComplexSexp};

#[cfg(feature = "complex")]
pub use savvy_ffi::Complex64;

pub use unwind_protect::unwind_protect;

pub use eval::{assert_eq_r_code, eval_parse_text, EvalResult};

// re-export
pub use savvy_macro::savvy;
pub use savvy_macro::savvy_init;

use ffi::SEXP;
use savvy_ffi::{cetype_t_CE_UTF8, Rf_allocVector, Rf_mkCharLenCE, SEXPTYPE};

fn alloc_vector(arg1: SEXPTYPE, arg2: usize) -> crate::error::Result<SEXP> {
    unsafe { unwind_protect(|| Rf_allocVector(arg1, arg2 as _)) }
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
    ($($arg:tt)*) => { $crate::io::r_print(&format!($($arg)*), false); };
}

#[macro_export]
macro_rules! r_eprint {
    () => {};
    ($($arg:tt)*) => { $crate::io::r_eprint(&format!($($arg)*), false); };
}

#[macro_export]
macro_rules! r_println {
    () => {};
    ($($arg:tt)*) => { $crate::io::r_print(&format!($($arg)*), true); };
}

#[macro_export]
macro_rules! r_eprintln {
    () => {};
    ($($arg:tt)*) => { $crate::io::r_eprint(&format!($($arg)*), true); };
}

#[macro_export]
macro_rules! savvy_err {
    () => {};
    ($($arg:tt)*) => { $crate::Error::new(&format!($($arg)*)) };
}
