use libR_sys::{REprintf, R_NilValue, Rf_translateCharUTF8, Rf_xlength, Rprintf, SEXP, STRING_ELT};
use std::ffi::{CStr, CString};

mod error;
mod sxp;

// TODO: make this r_println! macro
fn r_print(msg: String) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        Rprintf(msg_c_string.as_ptr());
    }
}

fn r_eprint(msg: String) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        REprintf(msg_c_string.as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_to_upper(x: SEXP) -> SEXP {
    let x = sxp::StringSxp::try_from(x).unwrap();

    for i in 0..x.len() {
        let e = &x[i];
        let e_upper = e.to_uppercase();
        r_eprint(format!("{e_upper}\n\n"));
    }

    R_NilValue
}

// #[no_mangle]
// pub unsafe extern "C" fn unextendr_set_car(x: SEXP, y: SEXP) -> SEXP {
//     let c = libR_sys::Rf_cons(x, libR_sys::R_NilValue);
//     libR_sys::SET_TAG(c, y);
//     c
// }
