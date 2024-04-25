mod mod1_1;

use savvy::{r_println, savvy};

#[savvy]
fn fun_mod1() -> savvy::Result<()> {
    r_println!("foo!");
    Ok(())
}
