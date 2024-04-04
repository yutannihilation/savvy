use savvy::{r_println, savvy};

/// A Or B.
///
/// @export
#[savvy]
#[derive(Debug)]
enum FooEnum {
    A,
    B,
}

#[savvy]
impl FooEnum {
    fn print(&self) -> savvy::Result<()> {
        r_println!("{:?}", self);
        Ok(())
    }
}

#[savvy]
fn print_foo_enum(x: FooEnum) -> savvy::Result<()> {
    x.print()
}

#[savvy]
fn print_foo_enum_ref(x: &FooEnum) -> savvy::Result<()> {
    x.print()
}

#[savvy]
fn foo_a() -> savvy::Result<FooEnum> {
    Ok(FooEnum::A)
}
