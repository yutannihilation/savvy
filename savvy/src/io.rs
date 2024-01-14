use savvy_ffi::{REprintf, Rprintf};

use once_cell::sync::Lazy;
use std::ffi::CString;

pub(crate) static LINEBREAK: Lazy<CString> = Lazy::new(|| CString::new("\n").unwrap());

pub fn r_print(msg: &str, linebreak: bool) {
    unsafe {
        if !msg.is_empty() {
            let msg_c_string = CString::new(msg).unwrap();
            Rprintf(msg_c_string.as_ptr());
        }

        if linebreak {
            Rprintf(LINEBREAK.as_ptr());
        }
    }
}

pub fn r_eprint(msg: &str, linebreak: bool) {
    unsafe {
        if !msg.is_empty() {
            let msg_c_string = CString::new(msg).unwrap();
            REprintf(msg_c_string.as_ptr());
        }

        if linebreak {
            REprintf(LINEBREAK.as_ptr());
        }
    }
}
