use savvy::savvy;

#[savvy]
fn is_obj(x: savvy::Sexp) -> savvy::Result<savvy::Sexp> {
    x.is_obj().try_into()
}

#[savvy]
fn get_obj_class(x: savvy::Sexp) -> savvy::Result<savvy::Sexp> {
    match x.into_typed() {
        savvy::TypedSexp::Obj(obj) => obj.get_class().unwrap_or(vec![]).try_into(),
        _ => ().try_into(),
    }
}
