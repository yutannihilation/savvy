use savvy::{savvy, EnvironmentSexp, Sexp};

#[savvy]
fn get_var_in_env(env: EnvironmentSexp, name: &str) -> savvy::Result<Sexp> {
    let obj = env.get(name)?;
    obj.ok_or("Not found".into())
}

#[savvy]
fn var_exists_in_env(env: EnvironmentSexp, name: &str) -> savvy::Result<Sexp> {
    env.contains(name)?.try_into()
}

#[savvy]
fn set_var_in_env(env: EnvironmentSexp, name: &str, value: Sexp) -> savvy::Result<()> {
    env.set(name, value)
}
