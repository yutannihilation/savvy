mod error;
mod integer;
mod logical;
mod na;
mod protect;
mod real;
mod sexp;
mod string;

mod unwind_protect_wrapper;

use integer::{IntegerSxp, OwnedIntegerSxp};
use libR_sys::{cetype_t_CE_UTF8, REprintf, Rf_mkCharLenCE, Rprintf, SEXP};
use logical::{LogicalSxp, OwnedLogicalSxp};
use na::NotAvailableValue;
use protect::{
    insert_to_preserved_list, release_from_preserved_list, PreservedList, PRESERVED_LIST,
};
use real::{OwnedRealSxp, RealSxp};
use std::ffi::CString;
use string::{OwnedStringSxp, StringSxp};

use unwind_protect_wrapper::unwind_protect;

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

unsafe fn to_upper_inner(x: SEXP) -> crate::error::Result<SEXP> {
    let x = StringSxp::try_from(x)?;
    let mut out = OwnedStringSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na());
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str());
    }

    Ok(out.inner())
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_to_upper(x: SEXP) -> SEXP {
    wrapper(|| to_upper_inner(x))
}

unsafe fn times_two_int_inner(x: SEXP) -> crate::error::Result<SEXP> {
    let x = IntegerSxp::try_from(x)?;
    let mut out = OwnedIntegerSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, i32::na());
        } else {
            out.set_elt(i, e * 2);
        }
    }

    Ok(out.inner())
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_int(x: SEXP) -> SEXP {
    wrapper(|| times_two_int_inner(x))
}

unsafe fn times_two_numeric_inner(x: SEXP) -> crate::error::Result<SEXP> {
    let x = RealSxp::try_from(x)?;
    let mut out = OwnedRealSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, f64::na())
        } else {
            out.set_elt(i, e * 2.0)
        }
    }

    Ok(out.inner())
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_numeric(x: SEXP) -> SEXP {
    wrapper(|| times_two_numeric_inner(x))
}

unsafe fn flip_logical_inner(x: SEXP) -> crate::error::Result<SEXP> {
    let x = LogicalSxp::try_from(x)?;
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e);
    }

    Ok(out.inner())
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_flip_logical(x: SEXP) -> SEXP {
    wrapper(|| flip_logical_inner(x))
}
