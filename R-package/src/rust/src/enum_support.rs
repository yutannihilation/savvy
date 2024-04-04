use std::ops::Deref;

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

const FOO_A: FooEnum = FooEnum::A;
const FOO_B: FooEnum = FooEnum::B;

impl TryFrom<savvy::Sexp> for &FooEnum {
    type Error = savvy::Error;

    fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
        let i = <i32>::try_from(value)?;
        match i {
            0 => Ok(&FOO_A),
            1 => Ok(&FOO_B),
            _ => Err("Unexpected enum variant".into()),
        }
    }
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
