use savvy::{
    savvy, sexp::function::FunctionSexp, EnvironmentSexp, ListSexp, OwnedIntegerSexp, Sexp,
};

#[savvy]
pub fn do_call(
    fun: FunctionSexp,
    args: ListSexp,
    env: EnvironmentSexp,
) -> savvy::Result<savvy::Sexp> {
    let res = fun.call(args.iter(), &env)?;
    res.into()
}

#[savvy]
pub fn call_with_args(fun: FunctionSexp, env: EnvironmentSexp) -> savvy::Result<savvy::Sexp> {
    let args = [
        ("a", Sexp::try_from(1)?),
        ("b", Sexp::try_from(2.0)?),
        ("c", Sexp::try_from("foo")?),
    ]
    .into_iter();
    let res = fun.call(args, &env)?;
    res.into()
}
