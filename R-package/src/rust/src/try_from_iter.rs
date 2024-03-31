use savvy::{
    savvy, ComplexSexp, IntegerSexp, LogicalSexp, NotAvailableValue, OwnedComplexSexp,
    OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp, OwnedStringSexp, RealSexp, Sexp, StringSexp,
};

#[savvy]
fn filter_integer_odd(x: IntegerSexp) -> savvy::Result<Sexp> {
    // is_na() is to propagate NAs
    let iter = x.iter().copied().filter(|i| i.is_na() || *i % 2 == 0);
    let out = OwnedIntegerSexp::try_from_iter(iter)?;
    out.into()
}

#[savvy]
fn filter_real_negative(x: RealSexp) -> savvy::Result<Sexp> {
    // is_na() is to propagate NAs
    let iter = x.iter().copied().filter(|r| r.is_na() || *r >= 0.0);
    let out = OwnedRealSexp::try_from_iter(iter)?;
    out.into()
}

#[savvy]
fn filter_complex_without_im(x: ComplexSexp) -> savvy::Result<Sexp> {
    // is_na() is to propagate NAs
    let iter = x.iter().copied().filter(|c| c.is_na() || c.im != 0.0);
    let out = OwnedComplexSexp::try_from_iter(iter)?;
    out.into()
}

#[savvy]
fn filter_logical_duplicates(x: LogicalSexp) -> savvy::Result<Sexp> {
    let mut last: Option<bool> = None;

    // Note: bool cannot represent NA, so NAs are just treated as TRUE
    let iter = x.iter().filter(|l| {
        let pred = match &mut last {
            // if the value is the same as the last one, discard it
            Some(v) => *l != *v,
            // first element is always kept
            None => true,
        };
        last = Some(*l);
        pred
    });
    let out = OwnedLogicalSexp::try_from_iter(iter)?;
    out.into()
}

#[savvy]
fn filter_string_ascii(x: StringSexp) -> savvy::Result<Sexp> {
    // is_na() is to propagate NAs
    let iter = x.iter().filter(|s| s.is_na() || s.is_ascii());
    let out = OwnedStringSexp::try_from_iter(iter)?;
    out.into()
}
