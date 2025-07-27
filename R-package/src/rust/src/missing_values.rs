use savvy::savvy;

#[savvy]
fn is_scalar_na(x: savvy::Sexp) -> savvy::Result<savvy::Sexp> {
    x.is_scalar_na().try_into()
}