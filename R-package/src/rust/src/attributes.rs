use savvy::savvy;

#[savvy]
fn get_class_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_class() {
        Some(class) => class.try_into(),
        None => ().try_into(),
    }
}

#[savvy]
fn get_names_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    x.get_names().try_into()
}

#[savvy]
fn get_dim_int(x: savvy::IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_dim() {
        Some(dim) => dim.iter().map(|i| *i as _).collect::<Vec<i32>>().try_into(),
        None => ().try_into(),
    }
}
