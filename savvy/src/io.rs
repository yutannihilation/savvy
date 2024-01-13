use savvy_ffi::{REprintf, Rprintf};

use std::ffi::CString;

use crate::unwind_protect;

pub fn r_print(msg: &str) -> crate::error::Result<()> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            Rprintf(msg_c_string.as_ptr());
            savvy_ffi::R_NilValue
        })?;

        Ok(())
    }
}

pub fn r_eprint(msg: &str) -> crate::error::Result<()> {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        unwind_protect(|| {
            REprintf(msg_c_string.as_ptr());
            savvy_ffi::R_NilValue
        })?;

        Ok(())
    }
}
