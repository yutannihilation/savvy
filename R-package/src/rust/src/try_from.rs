use savvy::savvy;

#[derive(Debug)]
struct MyInteger(i32);

impl TryFrom<savvy::Sexp> for MyInteger {
    type Error = savvy::Error;

    fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
        let i: i32 = value.try_into()?;
        Ok(Self(i))
    }
}

#[savvy]
fn my_integer(x: MyInteger) -> savvy::Result<()> {
    savvy::r_print!("{:?}\n", x);
    Ok(())
}
