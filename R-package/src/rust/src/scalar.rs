use savvy::{
    savvy, IntegerSexp, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp, OwnedStringSexp,
    RealSexp,
};

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
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    OwnedIntegerSexp::try_from(1).map(|x| x.into())
}

#[savvy]
fn scalar_output_real() -> savvy::Result<savvy::Sexp> {
    OwnedRealSexp::try_from(1.3).map(|x| x.into())
}

#[savvy]
fn scalar_output_logical() -> savvy::Result<savvy::Sexp> {
    OwnedLogicalSexp::try_from(false).map(|x| x.into())
}

#[savvy]
fn scalar_output_string() -> savvy::Result<savvy::Sexp> {
    OwnedStringSexp::try_from("foo").map(|x| x.into())
}

#[savvy]
fn sum_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let sum: OwnedIntegerSexp = x.as_slice().iter().sum::<i32>().try_into()?;
    Ok(sum.into())
}

#[savvy]
fn sum_real(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let sum: OwnedRealSexp = x.as_slice().iter().sum::<f64>().try_into()?;
    Ok(sum.into())
}
