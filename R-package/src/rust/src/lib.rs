use unextendr::sexp::integer::{IntegerSxp, OwnedIntegerSxp};
use unextendr::sexp::logical::{LogicalSxp, OwnedLogicalSxp};
use unextendr::sexp::na::NotAvailableValue;
use unextendr::sexp::real::{OwnedRealSxp, RealSxp};
use unextendr::sexp::string::{OwnedStringSxp, StringSxp};

use unextendr::unextendr;
use unextendr::SEXP;

#[unextendr]
unsafe fn to_upper(x: StringSxp) -> unextendr::error::Result<SEXP> {
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

#[unextendr]
unsafe fn times_two_int(x: IntegerSxp) -> unextendr::error::Result<SEXP> {
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

#[unextendr]
unsafe fn times_two_numeric(x: RealSxp) -> unextendr::error::Result<SEXP> {
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

#[unextendr]
unsafe fn flip_logical(x: LogicalSxp) -> unextendr::error::Result<SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e);
    }

    Ok(out.into())
}

#[unextendr]
fn foo_foo_foooo(x: i32, y: bool, z: RealSxp) -> unextendr::error::Result<SEXP> {
    let _ = 1 + 1;
    unextendr::sexp::null::NullSxp.into()
}
