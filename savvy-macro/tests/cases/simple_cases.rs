use savvy_macro::savvy;

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
