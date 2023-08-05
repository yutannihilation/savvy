use unextendr::sexp::integer::{IntegerSxp, OwnedIntegerSxp};
use unextendr::sexp::logical::{LogicalSxp, OwnedLogicalSxp};
use unextendr::sexp::na::NotAvailableValue;
use unextendr::sexp::real::{OwnedRealSxp, RealSxp};
use unextendr::sexp::string::{OwnedStringSxp, StringSxp};

use unextendr::unextendr;
use unextendr::SEXP;

unsafe fn to_upper_inner(x: SEXP) -> unextendr::error::Result<SEXP> {
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

    Ok(out.into())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_to_upper(x: SEXP) -> SEXP {
    unextendr::wrapper(|| to_upper_inner(x))
}

unsafe fn times_two_int_inner(x: SEXP) -> unextendr::error::Result<SEXP> {
    let x = IntegerSxp::try_from(x)?;
    let mut out = OwnedIntegerSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, i32::na());
        } else {
            out.set_elt(i, e * 2);
        }
    }

    Ok(out.into())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_int(x: SEXP) -> SEXP {
    unextendr::wrapper(|| times_two_int_inner(x))
}

unsafe fn times_two_numeric_inner(x: SEXP) -> unextendr::error::Result<SEXP> {
    let x = RealSxp::try_from(x)?;
    let mut out = OwnedRealSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, f64::na())
        } else {
            out.set_elt(i, e * 2.0)
        }
    }

    Ok(out.into())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_numeric(x: SEXP) -> SEXP {
    unextendr::wrapper(|| times_two_numeric_inner(x))
}

unsafe fn flip_logical_inner(x: SEXP) -> unextendr::error::Result<SEXP> {
    let x = LogicalSxp::try_from(x)?;
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e);
    }

    Ok(out.into())
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_flip_logical(x: SEXP) -> SEXP {
    unextendr::wrapper(|| flip_logical_inner(x))
}

#[unextendr]
fn foo_foo_foooo(x: i32, y: bool, z: RealSxp) -> unextendr::error::Result<SEXP> {
    let _ = 1 + 1;
    unextendr::sexp::null::NullSxp.into()
}
