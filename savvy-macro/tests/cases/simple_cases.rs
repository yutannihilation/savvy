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

struct Foo1;

#[savvy]
impl Foo1 {
    fn foo_self(&self) -> Self {
        Self {}
    }
}

struct Foo2;

#[savvy]
impl Foo2 {}

#[savvy]
fn wrong_type_custom_type_no_ref(x: Foo2) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
fn correct_type_custom_type_ref(x: &Foo2) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
fn correct_type_custom_type_ref_mut(x: &mut Foo2) -> savvy::Result<()> {
    Ok(())
}

fn main() {}
