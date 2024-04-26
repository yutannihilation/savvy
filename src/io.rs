use savvy_ffi::{REprintf, Rprintf};

use std::{ffi::CString, os::raw::c_char};

pub(crate) const LINEBREAK: [c_char; 2] = [b'\n' as _, b'\0' as _];

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
