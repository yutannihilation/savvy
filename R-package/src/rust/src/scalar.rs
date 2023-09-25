use savvy::savvy;

#[savvy]
fn scalar_input_int(x: i32) {
    savvy::r_print("Do nothing")?;
}

#[savvy]
fn scalar_input_real(x: f64) {
    savvy::r_print("Do nothing")?;
}

#[savvy]
fn scalar_input_logical(x: bool) {
    savvy::r_print("Do nothing")?;
}

#[savvy]
fn scalar_input_str(x: &str) {
    savvy::r_print("Do nothing")?;
}
