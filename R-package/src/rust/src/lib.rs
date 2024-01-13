#![allow(unused_variables)]

mod attributes;
pub use attributes::*;

mod convert_from_rust_types;
pub use convert_from_rust_types::*;

mod error_handling;
pub use error_handling::*;

mod try_from;
pub use try_from::*;

mod init_vectors;
pub use init_vectors::*;

use savvy::{r_print, savvy};

use savvy::{
    IntegerSexp, ListSexp, LogicalSexp, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp,
    OwnedStringSexp, RealSexp, StringSexp, TypedSexp,
};

use savvy::sexp::na::NotAvailableValue;

/// Convert Input To Upper-Case
///
/// @param x A character vector.
/// @returns A character vector with upper case version of the input.
/// @export
#[savvy]
fn to_upper(x: StringSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, &e_upper)?;
    }

    out.into()
}

/// Add suffix
///
/// @param x A character vector.
/// @param y A suffix.
/// @returns A character vector with upper case version of the input.
/// @export
#[savvy]
fn add_suffix(x: StringSexp, y: &str) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        out.set_elt(i, &format!("{e}_{y}"))?;
    }

    out.into()
}

/// Multiply Input By Two
///
/// @param x An integer vector.
/// @returns An integer vector with values multiplied by 2.
/// @export
#[savvy]
fn times_two_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = i32::na();
        } else {
            out[i] = e * 2;
        }
    }

    out.into()
}

/// Multiply Input By Another Input
///
/// @param x An integer vector.
/// @param y An integer to multiply.
/// @returns An integer vector with values multiplied by `y`.
/// @export
#[savvy]
fn times_any_int(x: IntegerSexp, y: i32) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = i32::na();
        } else {
            out[i] = e * y;
        }
    }

    out.into()
}

/// Multiply Input By Two
///
/// @param x A numeric vector.
/// @returns A numeric vector with values multiplied by 2.
/// @export
#[savvy]
fn times_two_numeric(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedRealSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = f64::na();
        } else {
            out[i] = e * 2.0;
        }
    }

    out.into()
}

/// Multiply Input By Another Input
///
/// @param x A real vector.
/// @param y A real to multiply.
/// @returns A real vector with values multiplied by `y`.
/// @export
#[savvy]
fn times_any_numeric(x: RealSexp, y: f64) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedRealSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out[i] = f64::na();
        } else {
            out[i] = e * y;
        }
    }

    out.into()
}

/// Flip Input
///
/// @param x A logical vector.
/// @returns A logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[savvy]
fn flip_logical(x: LogicalSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, !e)?;
    }

    out.into()
}

/// Or operation
///
/// @param x A logical vector.
/// @param y A logical value.
/// @returns A logical vector with filled values (`NA` is converted to `TRUE`).
/// @export
#[savvy]
fn or_logical(x: LogicalSexp, y: bool) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e || y)?;
    }

    out.into()
}

/// Print the content of list
///
/// @param x A list vector.
/// @returns `NULL`
/// @export
#[savvy]
fn print_list(x: ListSexp) -> savvy::Result<()> {
    for (k, v) in x.iter() {
        let content = match v {
            TypedSexp::Integer(x) => {
                format!(
                    "integer [{}]",
                    x.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypedSexp::Real(x) => {
                format!(
                    "numeric [{}]",
                    x.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypedSexp::String(x) => {
                format!("character [{}]", x.iter().collect::<Vec<&str>>().join(", "))
            }
            TypedSexp::Logical(x) => {
                format!(
                    "logical [{}]",
                    x.iter()
                        .map(|l| if l { "TRUE" } else { "FALSE" })
                        .collect::<Vec<&str>>()
                        .join(", ")
                )
            }
            TypedSexp::List(_) => "list".to_string(),
            TypedSexp::Null(_) => "NULL".to_string(),
            TypedSexp::Other(_) => "Unsupported".to_string(),
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

    fn name(&self) -> savvy::Result<savvy::Sexp> {
        let name = self.name.as_str();
        name.try_into()
    }

    fn associated_function() -> savvy::Result<savvy::Sexp> {
        "associated_function".try_into()
    }
}
