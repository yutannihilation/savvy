// This file is to check if savvy-cli can handle multiple definitions.

use savvy::savvy;

#[savvy]
#[cfg(target_os = "windows")]
fn fn_w_cfg(x: savvy::Sexp) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
#[cfg(not(target_os = "windows"))]
fn fn_w_cfg(x: savvy::Sexp) -> savvy::Result<()> {
    Ok(())
}
