#![allow(unused_variables)]

mod altrep;
mod attributes;
mod complex;
mod consuming_type;
mod convert_from_rust_types;
mod enum_support;
mod environment;
mod error_handling;
mod function;
mod init_vectors;
mod missing_values;
mod multiple_defs;
mod numeric;
mod optional_arg;
mod try_from_iter;

// This doesn't need r#, but this is to test if a raw identifier is handled correctly
mod r#escape;

// to test if the definition over multiple files is accepted.
// cf. https://github.com/yutannihilation/savvy/issues/118
mod separate_impl_definition;

// to test modules are parsed properly
// cf. https://github.com/yutannihilation/savvy/issues/147
mod mod1;

// This should not be parsed
// mod mod2;

mod log;

use savvy::{r_print, savvy, OwnedListSexp, OwnedRawSexp, RawSexp};

use savvy::{
    IntegerSexp, ListSexp, LogicalSexp, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp,
    OwnedStringSexp, RealSexp, StringSexp, TypedSexp,
};

use savvy::sexp::na::NotAvailableValue;

#[savvy]
fn is_built_with_debug() -> savvy::Result<savvy::Sexp> {
    cfg!(debug_assertions).try_into()
}

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
            out.set_na(i)?;
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
            out.set_na(i)?;
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
            out.set_na(i)?;
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
            out.set_na(i)?;
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
fn times_two_real(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedRealSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
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
fn times_any_real(x: RealSexp, y: f64) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedRealSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
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

// To handle NA values in a logical vector, use the raw values of i32, instead of bool.
#[savvy]
fn flip_logical_expert_only(x: LogicalSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(x.len())?;

    for (i, e) in x.as_slice_raw().iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
        } else {
            out.set_elt(i, *e != 1)?; // 1 means TRUE
        }
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

/// Reverse bits
///
/// @param x A raw vector.
#[savvy]
fn reverse_bits(x: RawSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedRawSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e.reverse_bits())?;
    }

    out.into()
}

/// Reverse bits
///
/// @param x A raw vector.
#[savvy]
fn reverse_bit_scalar(x: u8) -> savvy::Result<savvy::Sexp> {
    let out: u8 = x.reverse_bits();
    out.try_into()
}

/// Print the content of list
///
/// @param x A list vector.
/// @returns `NULL`
/// @export
#[savvy]
fn print_list(x: ListSexp) -> savvy::Result<()> {
    for (k, v) in x.iter() {
        let content = match v.into_typed() {
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
                    "double [{}]",
                    x.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypedSexp::Complex(x) => {
                format!(
                    "complex [{}]",
                    x.iter()
                        .map(|r| format!("{}+{}i", r.re, r.im))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
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
            TypedSexp::String(x) => {
                format!("character [{}]", x.iter().collect::<Vec<&str>>().join(", "))
            }
            TypedSexp::List(_) => "list".to_string(),
            TypedSexp::Null(_) => "NULL".to_string(),
            TypedSexp::ExternalPointer(_) => "external pointer".to_string(),
            TypedSexp::Function(_) => "function".to_string(),
            _ => "Unsupported".to_string(),
        };

        let name = if k.is_empty() { "(no name)" } else { k };

        r_print!("{name}: {content}\n");
    }

    Ok(())
}

#[savvy]
fn list_with_no_values() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, true)?;

    out.set_name(0, "foo")?;
    out.set_name(1, "bar")?;

    out.into()
}

#[savvy]
fn list_with_no_names() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, false)?;

    let mut e1 = OwnedIntegerSexp::new(1)?;
    e1[0] = 100;

    let mut e2 = OwnedStringSexp::new(1)?;
    e2.set_elt(0, "cool")?;

    out.set_value(0, e1)?;
    out.set_value(1, e2)?;

    out.into()
}

#[savvy]
fn list_with_names_and_values() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, true)?;

    let mut e1 = OwnedIntegerSexp::new(1)?;
    e1[0] = 100;

    let mut e2 = OwnedStringSexp::new(1)?;
    e2.set_elt(0, "cool")?;

    out.set_name_and_value(0, "foo", e1)?;
    out.set_name_and_value(1, "bar", e2)?;

    out.into()
}

/// A person with a name
///
/// @export
#[savvy]
struct Person {
    pub name: String,
}

#[savvy]
#[allow(dead_code)]
struct Person2 {
    pub name: String,
}

#[savvy]
impl Person {
    fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    // Allow the same type name as Self
    //
    // https://github.com/yutannihilation/savvy/issues/136
    fn new2() -> Person {
        Person {
            name: "".to_string(),
        }
    }

    fn new_fallible() -> savvy::Result<Self> {
        Ok(Self {
            name: "".to_string(),
        })
    }

    fn another_person(&self) -> savvy::Result<Person2> {
        Ok(Person2 {
            name: self.name.clone(),
        })
    }

    fn new_with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
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

#[savvy]
impl Person2 {
    fn name(&self) -> savvy::Result<savvy::Sexp> {
        let name = self.name.as_str();
        name.try_into()
    }
}

#[savvy]
fn external_person_new() -> savvy::Result<Person> {
    Ok(Person {
        name: "".to_string(),
    })
}

#[savvy]
fn get_name_external(x: &Person) -> savvy::Result<savvy::Sexp> {
    x.name()
}

#[savvy]
fn set_name_external(x: &mut Person, name: &str) -> savvy::Result<()> {
    x.set_name(name)
}

#[cfg(feature = "savvy-test")]
mod tests {
    #[test]
    fn test_to_upper() -> savvy::Result<()> {
        let x = savvy::OwnedStringSexp::try_from_slice(["foo", "bar", "BAZ", "ハート"])?;
        let result = super::to_upper(x.as_read_only())?;
        savvy::assert_eq_r_code(result, r#"c("FOO", "BAR", "BAZ", "ハート")"#);
        Ok(())
    }

    #[test]
    fn test_add_suffix() -> savvy::Result<()> {
        let x = savvy::OwnedStringSexp::try_from_slice(["foo", "bar"])?;
        let result = super::add_suffix(x.as_read_only(), "suf")?;
        savvy::assert_eq_r_code(result, r#"c("foo_suf", "bar_suf")"#);
        Ok(())
    }
}
