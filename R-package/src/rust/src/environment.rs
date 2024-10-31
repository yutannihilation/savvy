use savvy::{savvy, savvy_err, EnvironmentSexp, Sexp};

#[savvy]
fn get_var_in_env(name: &str, env: Option<EnvironmentSexp>) -> savvy::Result<Sexp> {
    let env = env.unwrap_or(EnvironmentSexp::global_env());
    let obj = env.get(name)?;
    obj.ok_or(savvy_err!("Not found"))
}

#[savvy]
fn var_exists_in_env(name: &str, env: Option<EnvironmentSexp>) -> savvy::Result<Sexp> {
    let env = env.unwrap_or(EnvironmentSexp::global_env());
    env.contains(name)?.try_into()
}

#[savvy]
fn set_var_in_env(name: &str, value: Sexp, env: Option<EnvironmentSexp>) -> savvy::Result<()> {
    let env = env.unwrap_or(EnvironmentSexp::global_env());
    env.set(name, value)
}
