use savvy::{
    r_println, savvy, NotAvailableValue, NumericScalar, NumericSexp, NumericTypedSexp,
    OwnedIntegerSexp, OwnedRealSexp, OwnedStringSexp, Sexp,
};

#[savvy]
fn times_two_numeric_f64(x: NumericSexp) -> savvy::Result<Sexp> {
    let mut out = OwnedRealSexp::new(x.len())?;

    for (i, v) in x.iter_f64().enumerate() {
        if v.is_na() {
            out[i] = f64::na();
        } else {
            out[i] = v * 2.0;
        }
    }

    out.into()
}

#[savvy]
fn times_two_numeric_i32(x: NumericSexp) -> savvy::Result<Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, v) in x.iter_i32().enumerate() {
        let v = v?;
        if v.is_na() {
            out[i] = i32::na();
        } else {
            out[i] = v * 2;
        }
    }

    out.into()
}

#[savvy]
fn usize_to_string(x: NumericSexp) -> savvy::Result<Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;
    for (i, v) in x.iter_usize().enumerate() {
        out.set_elt(i, &v?.to_string())?;
    }

    out.into()
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

#[savvy]
fn usize_to_string_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    x.as_usize()?.to_string().try_into()
}

#[savvy]
fn print_numeric(x: NumericSexp) -> savvy::Result<()> {
    match x.into_typed() {
        NumericTypedSexp::Integer(i) => {
            r_println!("Integer {:?}", i.as_slice());
        }
        NumericTypedSexp::Real(r) => {
            r_println!("Real {:?}", r.as_slice());
        }
    }
    Ok(())
}

// https://github.com/yutannihilation/savvy/issues/387
#[savvy]
fn is_numeric(x: Sexp) -> savvy::Result<Sexp> {
    x.is_numeric().try_into()
}
