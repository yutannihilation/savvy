use savvy::{savvy, FunctionArgs, FunctionSexp, ListSexp};

#[savvy]
pub fn do_call(fun: FunctionSexp, args: ListSexp) -> savvy::Result<savvy::Sexp> {
    let args = FunctionArgs::from_list(args)?;
    let res = fun.call(args)?;
    res.into()
}

#[savvy]
pub fn call_with_args(fun: FunctionSexp) -> savvy::Result<savvy::Sexp> {
    let mut args = FunctionArgs::new();
    args.add("a", 1)?;
    args.add("b", 2.0)?;
    args.add("c", "foo")?;
    let res = fun.call(args)?;
    res.into()
}

#[savvy]
pub fn get_args(args: ListSexp) -> savvy::Result<savvy::Sexp> {
    let args = FunctionArgs::from_list(args)?;
    Ok(savvy::Sexp(args.inner()))
}
