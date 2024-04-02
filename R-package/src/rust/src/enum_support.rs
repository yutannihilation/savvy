use savvy::{r_println, savvy};

/// @export
#[savvy]
#[derive(Debug)]
enum Foo {
    A = 0,
    B = 1,
}

#[savvy]
fn foo(x: Foo) -> savvy::Result<()> {
    r_println!("{:?}", x);
    Ok(())
}
