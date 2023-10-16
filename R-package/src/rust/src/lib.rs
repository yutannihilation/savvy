#![allow(unused_variables)]

mod scalar;
pub use scalar::*;

mod error_handling;
pub use error_handling::*;

use savvy::{r_print, savvy};

use savvy::{
    IntegerSxp, ListElement, ListSxp, LogicalSxp, OwnedIntegerSxp, OwnedLogicalSxp, OwnedRealSxp,
    OwnedStringSxp, RealSxp, StringSxp,
};

use savvy::sexp::na::NotAvailableValue;

/// Convert Input To Upper-Case
///
/// @param x A character vector.
/// @returns A character vector with upper case version of the input.
/// @export
#[savvy]
fn to_upper(x: StringSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, &e_upper)?;
    }

    Ok(out.into())
}

/// Add suffix
///
/// @param x A character vector.
/// @param y A suffix.
/// @returns A character vector with upper case version of the input.
/// @export
#[savvy]
fn add_suffix(x: StringSxp, y: &str) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        out.set_elt(i, &format!("{e}_{y}"))?;
    }

    Ok(out.into())
}

/// Multiply Input By Two
///
/// @param x An integer vector.
/// @returns An integer vector with values multiplied by 2.
/// @export
#[savvy]
fn times_two_int(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = i32::na();
        } else {
            out[i] = e * 2;
        }
    }

    Ok(out.into())
}

/// Multiply Input By Another Input
///
/// @param x An integer vector.
/// @param y An integer to multiply.
/// @returns An integer vector with values multiplied by `y`.
/// @export
#[savvy]
fn times_any_int(x: IntegerSxp, y: i32) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = i32::na();
        } else {
            out[i] = e * y;
        }
    }

    Ok(out.into())
}

/// Multiply Input By Two
///
/// @param x A numeric vector.
/// @returns A numeric vector with values multiplied by 2.
/// @export
#[savvy]
fn times_two_numeric(x: RealSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedRealSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = f64::na();
        } else {
            out[i] = e * 2.0;
        }
    }

    Ok(out.into())
}

/// Multiply Input By Another Input
///
/// @param x A real vector.
/// @param y A real to multiply.
/// @returns A real vector with values multiplied by `y`.
/// @export
#[savvy]
fn times_any_numeric(x: RealSxp, y: f64) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedRealSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = f64::na();
        } else {
            out[i] = e * y;
        }
    }

    Ok(out.into())
}

/// Flip Input
///
/// @param x A logical vector.
/// @returns A logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[savvy]
fn flip_logical(x: LogicalSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e)?;
    }

    Ok(out.into())
}

/// Or operation
///
/// @param x A logical vector.
/// @param y A logical value.
/// @returns A logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[savvy]
fn or_logical(x: LogicalSxp, y: bool) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e || y)?;
    }

    Ok(out.into())
}

/// Print the content of list
///
/// @param x A list vector.
/// @returns `NULL`
/// @export
#[savvy]
fn print_list(x: ListSxp) -> savvy::Result<()> {
    for (k, v) in x.iter() {
        let content = match v {
            ListElement::Integer(x) => {
                format!(
                    "integer [{}]",
                    x.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ListElement::Real(x) => {
                format!(
                    "numeric [{}]",
                    x.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ListElement::String(x) => {
                format!("character [{}]", x.iter().collect::<Vec<&str>>().join(", "))
            }
            ListElement::Logical(x) => {
                format!(
                    "logical [{}]",
                    x.iter()
                        .map(|l| if l { "TRUE" } else { "FALSE" })
                        .collect::<Vec<&str>>()
                        .join(", ")
                )
            }
            ListElement::List(_) => "list".to_string(),
            ListElement::Null(_) => "NULL".to_string(),
            ListElement::Unsupported(_) => "Unsupported".to_string(),
        };

        let name = if k.is_empty() { "(no name)" } else { k };

        r_print(format!("{name}: {content}\n").as_str())?;
    }

    Ok(())
}

struct Person {
    pub name: String,
}

/// A person with a name
///
/// @export
#[savvy]
impl Person {
    fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    fn set_name(&mut self, name: &str) -> savvy::Result<()> {
        self.name = name.to_string();
        Ok(())
    }

    fn name(&self) -> savvy::Result<savvy::SEXP> {
        let mut out = OwnedStringSxp::new(1)?;
        out.set_elt(0, &self.name)?;
        Ok(out.into())
    }

    fn associated_function() -> savvy::Result<savvy::SEXP> {
        let mut out = OwnedStringSxp::new(1)?;
        out.set_elt(0, "associated_function")?;
        Ok(out.into())
    }
}
