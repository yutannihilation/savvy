use savvy::{savvy, sexp::function::FunctionSexp, EnvironmentSexp};

#[savvy]
pub fn do_call(fun: FunctionSexp, env: EnvironmentSexp) -> savvy::Result<savvy::Sexp> {
    fun.call(&env)
}
