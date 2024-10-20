use savvy::savvy;
use savvy::NotAvailableValue;

#[savvy]
fn new_complex(size: i32) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedComplexSexp::new(size as usize)?.into()
}

#[savvy]
fn first_complex(x: savvy::ComplexSexp) -> savvy::Result<savvy::Sexp> {
    let x_first = x.as_slice()[0];
    x_first.try_into()
}

#[savvy]
fn abs_complex(x: savvy::ComplexSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = savvy::OwnedRealSexp::new(x.len())?;

    for (i, c) in x.iter().enumerate() {
        if !c.is_na() {
            out[i] = (c.re * c.re + c.im * c.im).sqrt();
        } else {
            out.set_na(i)?;
        }
    }

    out.into()
}
