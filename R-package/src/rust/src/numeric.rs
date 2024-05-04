use savvy::{savvy, sexp::numeric::NumericSexp, NotAvailableValue, Sexp};

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
