use savvy::{
    savvy,
    sexp::numeric::{NumericScalar, NumericSexp},
    NotAvailableValue, Sexp,
};

#[savvy]
fn times_two_numeric_f64(x: NumericSexp) -> savvy::Result<Sexp> {
    let out: Vec<f64> = x
        .iter_f64()
        .map(|v| if v.is_na() { f64::na() } else { *v * 2.0 })
        .collect();
    out.try_into()
}

#[savvy]
fn times_two_numeric_i32(x: NumericSexp) -> savvy::Result<Sexp> {
    let out: Vec<i32> = x
        .iter_i32()?
        .map(|v| if v.is_na() { i32::na() } else { *v * 2 })
        .collect();
    out.try_into()
}

#[savvy]
fn times_two_numeric_f64_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    let v = x.as_f64();
    if v.is_na() {
        (f64::na()).try_into()
    } else {
        (v * 2.0).try_into()
    }
}

#[savvy]
fn times_two_numeric_i32_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    let v = x.as_i32()?;
    if v.is_na() {
        (i32::na()).try_into()
    } else {
        (v * 2).try_into()
    }
}
