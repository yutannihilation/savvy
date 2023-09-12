use std::ops::{Deref, DerefMut};

use unextendr::unextendr;

use unextendr::{
    IntegerSxp, ListSxp, LogicalSxp, NullSxp, OwnedIntegerSxp, OwnedLogicalSxp, OwnedRealSxp,
    OwnedStringSxp, RealSxp, StringSxp,
};

use unextendr::sexp::na::NotAvailableValue;
use unextendr::IntoExtPtrSxp;

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

struct Person {
    pub name: String,
}

/// A person with a name
///
/// @export
#[unextendr]
impl Person {
    fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    fn set_name(&mut self, name: StringSxp) {
        self.name = name.iter().next().unwrap().to_string();
    }

    fn name(&self) -> unextendr::Result<unextendr::SEXP> {
        let mut out = OwnedStringSxp::new(1);
        out.set_elt(0, self.name.as_str());
        Ok(out.into())
    }
}

// impl IntoExtPtrSxp for Person {}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_Person_new() -> unextendr::SEXP {
    unextendr::handle_result(unextendr_Person_new_inner())
}

unsafe fn unextendr_Person_new_inner() -> unextendr::Result<unextendr::SEXP> {
    let x = Person::new();
    Ok(x.into_external_pointer())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_Person_set_name(
    self__: unextendr::SEXP,
    name: unextendr::SEXP,
) -> unextendr::SEXP {
    unextendr::handle_result(unextendr_Person_set_name_inner(self__, name))
}

unsafe fn unextendr_Person_set_name_inner(
    self__: unextendr::SEXP,
    name: unextendr::SEXP,
) -> unextendr::Result<unextendr::SEXP> {
    let self__ = unextendr::get_external_pointer_addr(self__) as *mut Person;
    let name = unextendr::StringSxp::try_from(name)?;

    (*self__).set_name(name);

    Ok(NullSxp.into())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn unextendr_Person_name(self__: unextendr::SEXP) -> unextendr::SEXP {
    unextendr::handle_result(unextendr_Person_name_inner(self__))
}

unsafe fn unextendr_Person_name_inner(
    self__: unextendr::SEXP,
) -> unextendr::Result<unextendr::SEXP> {
    let self__ = unextendr::get_external_pointer_addr(self__) as *mut Person;

    (*self__).name()
}
