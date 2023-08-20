use unextendr::sexp::integer::{IntegerSxp, OwnedIntegerSxp};
use unextendr::sexp::list::ListSxp;
use unextendr::sexp::logical::{LogicalSxp, OwnedLogicalSxp};
use unextendr::sexp::na::NotAvailableValue;
use unextendr::sexp::real::{OwnedRealSxp, RealSxp};
use unextendr::sexp::string::{OwnedStringSxp, StringSxp};

use unextendr::{unextendr, NullSxp};

/// Convert Input To Upper-Case
///
/// @param x A character vector.
/// @returns A character vector with upper case version of the input.
/// @export
#[unextendr]
fn to_upper(x: StringSxp) -> unextendr::Result<unextendr::SEXP> {
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
fn times_two_int(x: IntegerSxp) -> unextendr::Result<unextendr::SEXP> {
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
fn times_two_numeric(x: RealSxp) -> unextendr::Result<unextendr::SEXP> {
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
/// @param x A logical vector.
/// @returns A logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[unextendr]
fn flip_logical(x: LogicalSxp) -> unextendr::Result<unextendr::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e);
    }

    Ok(out.into())
}

/// Print the content of list
///
/// @param x A list vector.
/// @returns `NULL`
/// @export
#[unextendr]
fn print_list(x: ListSxp) {
    for (k, v) in x.iter() {
        let content = match v {
            unextendr::sexp::list::ListElement::Integer(x) => {
                format!(
                    "integer [{}]",
                    x.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            unextendr::sexp::list::ListElement::Real(x) => {
                format!(
                    "numeric [{}]",
                    x.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            unextendr::sexp::list::ListElement::String(x) => {
                format!("character [{}]", x.iter().collect::<Vec<&str>>().join(", "))
            }
            unextendr::sexp::list::ListElement::Logical(x) => {
                format!(
                    "logical [{}]",
                    x.iter()
                        .map(|l| if l { "TRUE" } else { "FALSE" })
                        .collect::<Vec<&str>>()
                        .join(", ")
                )
            }
            unextendr::sexp::list::ListElement::List(_) => "list".to_string(),
            unextendr::sexp::list::ListElement::Null(_) => "NULL".to_string(),
            unextendr::sexp::list::ListElement::Unsupported(_) => "Unsupported".to_string(),
        };

        let name = if k.is_empty() { "(no name)" } else { k };

        unextendr::r_print(format!("{name}: {content}\n"));
    }
}
