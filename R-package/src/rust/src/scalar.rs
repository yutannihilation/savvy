use savvy::{savvy, IntegerSxp, OwnedIntegerSxp, OwnedLogicalSxp, OwnedRealSxp, OwnedStringSxp};

#[savvy]
fn scalar_input_int(x: i32) -> savvy::Result<()> {
    savvy::r_print(&format!("{}\n", x))?;
    Ok(())
}

#[savvy]
fn scalar_input_real(x: f64) -> savvy::Result<()> {
    savvy::r_print(&format!("{}\n", x))?;
    Ok(())
}

#[savvy]
fn scalar_input_logical(x: bool) -> savvy::Result<()> {
    savvy::r_print(&format!("{}\n", x))?;
    Ok(())
}

#[savvy]
fn scalar_input_string(x: &str) -> savvy::Result<()> {
    savvy::r_print(&format!("{}\n", x))?;
    Ok(())
}

#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::SEXP> {
    OwnedIntegerSxp::try_from(1).map(|x| x.into())
}

#[savvy]
fn scalar_output_real() -> savvy::Result<savvy::SEXP> {
    OwnedRealSxp::try_from(1.3).map(|x| x.into())
}

#[savvy]
fn scalar_output_logical() -> savvy::Result<savvy::SEXP> {
    OwnedLogicalSxp::try_from(false).map(|x| x.into())
}

#[savvy]
fn scalar_output_string() -> savvy::Result<savvy::SEXP> {
    OwnedStringSxp::try_from("foo").map(|x| x.into())
}

#[savvy]
fn sum_int(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let sum: OwnedIntegerSxp = x.as_slice().iter().sum::<i32>().try_into()?;
    Ok(sum.into())
}

#[savvy]
fn sum_real(x: RealSxp) -> savvy::Result<savvy::SEXP> {
    let sum: OwnedRealSxp = x.as_slice().iter().sum::<f64>().try_into()?;
    Ok(sum.into())
}
