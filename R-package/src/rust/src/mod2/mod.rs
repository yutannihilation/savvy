use savvy::{r_println, savvy};

#[savvy]
fn fun_mod2() -> savvy::Result<()> {
    r_println!("bar!");
    Ok(())
}
