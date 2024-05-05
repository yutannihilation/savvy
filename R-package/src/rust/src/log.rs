use savvy::savvy_init;
use savvy_ffi::DllInfo;

#[savvy_init]
fn init_logger(dll_info: *mut DllInfo) -> savvy::Result<()> {
    savvy::log::env_logger().init();
    Ok(())
}
