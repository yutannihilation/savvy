use savvy::{savvy, savvy_init, NullSexp, Sexp};
use savvy_ffi::DllInfo;
use std::ffi::CString;

use std::sync::{Mutex, OnceLock};

static FOO_VALUE: OnceLock<Mutex<i32>> = OnceLock::new();

#[savvy_init]
fn init_foo_value(dll: *mut DllInfo) -> savvy::Result<()> {
    match FOO_VALUE.set(Mutex::new(0)) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to set values".into()),
    }
}

struct Foo {}

impl Foo {
    fn new() -> Self {
        let v = FOO_VALUE.get().unwrap();
        *v.lock().unwrap() = 1;
        Foo {}
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        let v = FOO_VALUE.get().unwrap();
        *v.lock().unwrap() = 0;

        // If Foo is dropped, this message should be emmited.
        savvy::r_println!("Foo is Dropped!");
    }
}

#[savvy]
fn get_foo_value() -> savvy::Result<Sexp> {
    match FOO_VALUE.get() {
        Some(x) => {
            let v = *x.lock().unwrap();
            v.try_into()
        }
        None => NullSexp.into(),
    }
}

#[savvy]
fn safe_stop() -> savvy::Result<()> {
    let _ = Foo::new();

    unsafe {
        savvy::unwind_protect::unwind_protect(|| {
            let msg = CString::new("This is an error from inside unwind_protect()!").unwrap();
            savvy_ffi::Rf_errorcall(savvy_ffi::R_NilValue, msg.as_ptr());
        })?;
    }

    Ok(())
}

#[savvy]
fn raise_error() -> savvy::Result<savvy::Sexp> {
    Err(savvy::Error::new("This is my custom error"))
}

#[allow(clippy::out_of_bounds_indexing)]
#[savvy]
fn must_panic() -> savvy::Result<()> {
    let x = &[1];
    let _ = x[1];
    Ok(())
}

#[savvy]
fn safe_warn() -> savvy::Result<()> {
    let _ = Foo::new();

    savvy::io::r_warn("foo")?;

    Ok(())
}
