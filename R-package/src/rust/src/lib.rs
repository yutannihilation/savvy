use unextendr::sexp::integer::{IntegerSxp, OwnedIntegerSxp};
use unextendr::sexp::logical::{LogicalSxp, OwnedLogicalSxp};
use unextendr::sexp::na::NotAvailableValue;
use unextendr::sexp::real::{OwnedRealSxp, RealSxp};
use unextendr::sexp::string::{OwnedStringSxp, StringSxp};

use unextendr::unextendr;

/// Convert Input To Upper-Case
///
/// @param x A character vector.
/// @returns A character vector with upper case version of the input.
/// @export
#[unextendr]
unsafe fn to_upper(x: StringSxp) -> unextendr::Result<unextendr::SEXP> {
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

/// Multiply Input By Two
///
/// @param x An integer vector.
/// @returns An integer vector with values multiplied by 2.
/// @export
#[unextendr]
unsafe fn times_two_int(x: IntegerSxp) -> unextendr::Result<unextendr::SEXP> {
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

/// Multiply Input By Two
///
/// @param x A numeric vector.
/// @returns A numeric vector with values multiplied by 2.
/// @export
#[unextendr]
unsafe fn times_two_numeric(x: RealSxp) -> unextendr::Result<unextendr::SEXP> {
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

/// Flip Input
///
/// @param x An logical vector.
/// @returns An logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[unextendr]
unsafe fn flip_logical(x: LogicalSxp) -> unextendr::Result<unextendr::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e);
    }

    Ok(out.into())
}
