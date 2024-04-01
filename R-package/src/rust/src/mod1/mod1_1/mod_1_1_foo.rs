use savvy::{r_println, savvy};

#[savvy]
fn fun_mod1_1_foo() -> savvy::Result<()> {
    r_println!("foo!");
    Ok(())
}
