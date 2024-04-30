#![allow(unused_variables)]

use savvy_macro::savvy;
use savvy_macro::savvy_init;

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

#[savvy]
fn wrong_type_dllinfo(x: *mut DllInfo) -> savvy::Result<()> {
    Ok(())
}

// only fieldless enums is supported
#[savvy]
enum Foo {
    A(i32),
    B(&str),
}

// discreminant is not supported
#[savvy]
enum Foo {
    A,
    B = 100,
}

#[savvy_init]
fn init_wrong_type(x: DllInfo) -> savvy::Result<()> {
    Ok(())
}

#[savvy_init]
fn init_wrong_type2(x: *const DllInfo) -> savvy::Result<()> {
    Ok(())
}

#[savvy_init]
fn init_wrong_type3(x: *mut DllInfo, y: i32) -> savvy::Result<()> {
    Ok(())
}

fn main() {}
