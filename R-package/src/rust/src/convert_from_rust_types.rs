use savvy::{savvy, IntegerSexp, RealSexp};

// Scalar input, no out

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

// No input, scalar out

#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    1.try_into()
}

#[savvy]
fn scalar_output_real() -> savvy::Result<savvy::Sexp> {
    1.3.try_into()
}

#[savvy]
fn scalar_output_logical() -> savvy::Result<savvy::Sexp> {
    false.try_into()
}

#[savvy]
fn scalar_output_string() -> savvy::Result<savvy::Sexp> {
    "foo".try_into()
}

// Vector input, scalar out

#[savvy]
fn sum_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let sum: i32 = x.as_slice().iter().sum();
    sum.try_into()
}

#[savvy]
fn sum_real(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let sum: f64 = x.as_slice().iter().sum();
    sum.try_into()
}

// Scalar input, vector out

#[savvy]
fn rep_int_vec(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<i32> = std::iter::repeat(0).take(x as usize).collect();
    result.try_into()
}

#[savvy]
fn rep_int_slice(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<i32> = std::iter::repeat(0).take(x as usize).collect();
    result.as_slice().try_into()
}

#[savvy]
fn rep_real_vec(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<f64> = std::iter::repeat(0.0).take(x as usize).collect();
    result.try_into()
}

#[savvy]
fn rep_real_slice(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<f64> = std::iter::repeat(0.0).take(x as usize).collect();
    result.as_slice().try_into()
}

#[savvy]
fn rep_bool_vec(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<bool> = std::iter::repeat(true).take(x as usize).collect();
    result.try_into()
}

#[savvy]
fn rep_bool_slice(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<bool> = std::iter::repeat(true).take(x as usize).collect();
    result.as_slice().try_into()
}

#[savvy]
fn rep_str_vec(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<&str> = std::iter::repeat("foo").take(x as usize).collect();
    result.try_into()
}

#[savvy]
fn rep_str_slice(x: i32) -> savvy::Result<savvy::Sexp> {
    let result: Vec<&str> = std::iter::repeat("foo").take(x as usize).collect();
    result.as_slice().try_into()
}
