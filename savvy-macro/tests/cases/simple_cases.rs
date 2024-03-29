#![allow(unused_variables)]

use savvy_macro::savvy;

#[savvy]
fn no_return_type(x: i32) {}

#[savvy]
fn wrong_return_type(x: i32) -> i32 {}

#[savvy]
fn wrong_type_owned_int(x: OwnedIntegerSexp) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
fn wrong_type_owned_real(x: OwnedRealSexp) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
fn wrong_type_owned_logical(x: OwnedLogicalSexp) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
fn wrong_type_owned_string(x: OwnedStringSexp) -> savvy::Result<()> {
    Ok(())
}

fn main() {}
