use savvy::{savvy, OwnedIntegerSexp};

#[savvy]
fn get_class_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_class() {
        Some(class) => class.try_into(),
        None => ().try_into(),
    }
}

#[savvy]
fn get_names_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_names() {
        Some(names) => names.try_into(),
        None => ().try_into(),
    }
}

#[savvy]
fn get_dim_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_dim() {
        Some(dim) => {
            let x: OwnedIntegerSexp = dim.to_vec().try_into()?;
            x.into()
        }
        None => ().try_into(),
    }
}

#[savvy]
fn get_attr_int(x: savvy::IntegerSexp, attr: &str) -> savvy::Result<savvy::Sexp> {
    match x.get_attrib(attr)? {
        Some(attr) => Ok(attr),
        None => ().try_into(),
    }
}

#[savvy]
fn set_class_int() -> savvy::Result<savvy::Sexp> {
    let mut x = OwnedIntegerSexp::new(1)?;

    x.set_class(["foo", "bar"])?;

    x.into()
}

#[savvy]
fn set_names_int() -> savvy::Result<savvy::Sexp> {
    let x_vec = vec![1, 2];
    let mut x: OwnedIntegerSexp = x_vec.try_into()?;

    x.set_names(["foo", "bar"])?;

    x.into()
}

#[savvy]
fn set_dim_int() -> savvy::Result<savvy::Sexp> {
    let x_vec = vec![1, 2, 3, 4, 5, 6];
    let mut x: OwnedIntegerSexp = x_vec.try_into()?;

    x.set_dim(&[2, 3])?;

    x.into()
}

#[savvy]
fn set_attr_int(attr: &str, value: savvy::Sexp) -> savvy::Result<savvy::Sexp> {
    let mut x = OwnedIntegerSexp::new(1)?;

    x.set_attrib(attr, value)?;

    x.into()
}
