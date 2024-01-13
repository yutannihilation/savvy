use savvy::savvy;

#[savvy]
fn new_int(size: usize) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedIntegerSexp::new(size)?.into()
}

#[savvy]
fn new_real(size: usize) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedRealSexp::new(size)?.into()
}

#[savvy]
fn new_bool(size: usize) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedLogicalSexp::new(size)?.into()
}
