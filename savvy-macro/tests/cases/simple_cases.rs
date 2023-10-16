use savvy_macro::savvy;

#[savvy]
fn foo_ref(&x: i32) -> savvy::Result<()> {}

#[savvy]
fn foo_mut_ref(&mut x: i32) -> savvy::Result<()> {}

#[savvy]
fn foo_unsupported1(x: usize) -> savvy::Result<()> {}

#[savvy]
fn foo_no_return_type(x: i32) {}

#[savvy]
fn foo_wrong_return_type(x: i32) -> i32 {}

struct Foo;

#[savvy]
impl Foo {
    fn foo_self(&self) -> Self {}
}

fn main() {}
