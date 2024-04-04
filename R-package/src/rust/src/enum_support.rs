use savvy::{r_println, savvy};

/// A Or B.
///
/// @export
#[savvy]
#[derive(Debug)]
enum FooEnum {
    A = 0,
    B = 1,
}

#[savvy]
impl FooEnum {
    fn print(&self) -> savvy::Result<()> {
        r_println!("{:?}", self);
        Ok(())
    }
}

#[savvy]
fn foo(x: FooEnum) -> savvy::Result<()> {
    x.print()
}

#[savvy]
fn foo_a() -> savvy::Result<FooEnum> {
    Ok(FooEnum::A)
}
