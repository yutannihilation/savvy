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

#[savvy]
struct StructWithConfig(i32);

#[savvy]
impl StructWithConfig {
    #[cfg(target_os = "windows")]
    fn new_method(&self, x: i32) -> savvy::Result<Self> {
        Ok(Self(x))
    }

    #[cfg(not(target_os = "windows"))]
    fn new_method(&self, x: i32) -> savvy::Result<Self> {
        Ok(Self(x * 2))
    }

    #[cfg(target_os = "windows")]
    fn new_associated_fn(x: i32) -> savvy::Result<Self> {
        Ok(Self(x))
    }

    #[cfg(not(target_os = "windows"))]
    fn new_associated_fn(x: i32) -> savvy::Result<Self> {
        Ok(Self(x * 2))
    }
}
