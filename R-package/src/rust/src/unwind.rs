use savvy::savvy;
use std::ffi::CString;

struct Foo {}
impl Drop for Foo {
    fn drop(&mut self) {
        // If Foo is dropped, this message should be emmited.
        savvy::r_print("Foo is Dropped!\n");
    }
}

#[savvy]
fn safe_stop() {
    let _ = Foo {};

    savvy::unwind_protect::unwind_protect(|| unsafe {
        let msg = CString::new("Error!").unwrap();
        libR_sys::Rf_error(msg.as_ptr());
    })?;
}
