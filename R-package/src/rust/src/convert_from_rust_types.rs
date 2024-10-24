use savvy::{
    savvy, IntegerSexp, OwnedComplexSexp, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp,
    OwnedStringSexp, RealSexp,
};

// Scalar input, no out

#[savvy]
fn scalar_input_int(x: i32) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}

#[savvy]
fn scalar_input_real(x: f64) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}

#[savvy]
fn scalar_input_logical(x: bool) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}

#[savvy]
fn scalar_input_string(x: &str) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}

// No input, scalar out

#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    1.try_into()
}

#[savvy]
fn scalar_output_int2() -> savvy::Result<savvy::Sexp> {
    OwnedIntegerSexp::try_from_scalar(1)?.into()
}

#[savvy]
fn scalar_output_real() -> savvy::Result<savvy::Sexp> {
    1.3.try_into()
}

#[savvy]
fn scalar_output_real2() -> savvy::Result<savvy::Sexp> {
    OwnedRealSexp::try_from_scalar(1.3)?.into()
}

#[savvy]
fn scalar_output_logical() -> savvy::Result<savvy::Sexp> {
    false.try_into()
}

#[savvy]
fn scalar_output_logical2() -> savvy::Result<savvy::Sexp> {
    OwnedLogicalSexp::try_from_scalar(false)?.into()
}

#[savvy]
fn scalar_output_string() -> savvy::Result<savvy::Sexp> {
    "foo".try_into()
}

#[savvy]
fn scalar_output_string2() -> savvy::Result<savvy::Sexp> {
    OwnedStringSexp::try_from_scalar("foo")?.into()
}

#[savvy]
fn scalar_output_complex() -> savvy::Result<savvy::Sexp> {
    savvy::Complex64 { re: 1.0, im: 1.0 }.try_into()
}

#[savvy]
fn scalar_output_complex2() -> savvy::Result<savvy::Sexp> {
    let x = savvy::Complex64 { re: 1.0, im: 1.0 };
    OwnedComplexSexp::try_from_scalar(x)?.into()
}

// Vector input, scalar out

#[savvy]
fn sum_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let sum: i32 = x.iter().sum();
    sum.try_into()
}

#[savvy]
fn sum_real(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let sum: f64 = x.iter().sum();
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
