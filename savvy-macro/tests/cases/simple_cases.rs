use savvy_macro::savvy;

#[savvy]
fn foo_no_return_type(x: i32) {}

#[savvy]
fn foo_wrong_return_type(x: i32) -> i32 {}

#[savvy]
fn foo_wrong_type_owned_int(x: OwnedIntegerSexp) {}

#[savvy]
fn foo_wrong_type_owned_real(x: OwnedRealSexp) {}

#[savvy]
fn foo_wrong_type_owned_logical(x: OwnedLogicalSexp) {}

#[savvy]
fn foo_wrong_type_owned_string(x: OwnedStringSexp) {}

struct Foo;

#[savvy]
impl Foo {
    fn foo_self(&self) -> Self {}
}

fn main() {}
