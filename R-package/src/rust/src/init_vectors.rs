use savvy::savvy;

#[savvy]
fn new_int(size: i32) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedIntegerSexp::new(size as usize)?.into()
}

#[savvy]
fn new_real(size: i32) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedRealSexp::new(size as usize)?.into()
}

#[savvy]
fn new_bool(size: i32) -> savvy::Result<savvy::Sexp> {
    savvy::OwnedLogicalSexp::new(size as usize)?.into()
}
