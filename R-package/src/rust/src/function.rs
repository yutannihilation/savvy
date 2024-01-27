use savvy::{savvy, EnvironmentSexp, FunctionArgs, FunctionSexp, ListSexp};

#[savvy]
pub fn do_call(
    fun: FunctionSexp,
    args: ListSexp,
    env: EnvironmentSexp,
) -> savvy::Result<savvy::Sexp> {
    let args = FunctionArgs::from_list(args)?;
    let res = fun.call(args, &env)?;
    res.into()
}

#[savvy]
pub fn call_with_args(fun: FunctionSexp, env: EnvironmentSexp) -> savvy::Result<savvy::Sexp> {
    let mut args = FunctionArgs::new();
    args.add("a", 1)?;
    args.add("b", 2.0)?;
    args.add("c", "foo")?;
    let res = fun.call(args, &env)?;
    res.into()
}

#[savvy]
pub fn get_args(args: ListSexp) -> savvy::Result<savvy::Sexp> {
    let args = FunctionArgs::from_list(args)?;
    Ok(savvy::Sexp(args.inner()))
}
