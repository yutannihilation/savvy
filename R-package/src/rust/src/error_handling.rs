use savvy::savvy;
use std::ffi::CString;

struct Foo {}
impl Drop for Foo {
    fn drop(&mut self) {
        // If Foo is dropped, this message should be emmited.
        savvy::r_println!("Foo is Dropped!");
    }
}

#[savvy]
fn safe_stop() -> savvy::Result<()> {
    let _ = Foo {};

    savvy::unwind_protect::unwind_protect(|| unsafe {
        let msg = CString::new("Error!").unwrap();
        savvy_ffi::Rf_errorcall(savvy_ffi::R_NilValue, msg.as_ptr());
    })?;

    Ok(())
}

#[savvy]
fn raise_error() -> savvy::Result<savvy::Sexp> {
    Err(savvy::Error::new("This is my custom error"))
}

#[savvy]
fn must_panic() -> savvy::Result<()> {
    let x = &[1];
    let _ = x[1];
    Ok(())
}
