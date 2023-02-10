use std::ffi::{CStr, CString};

use libR_sys::{
    REprintf, R_NilValue, Rf_isString, Rf_translateCharUTF8, Rf_xlength, Rprintf, SEXP, STRING_ELT,
};

fn is_string(sexp: SEXP) -> bool {
    unsafe { Rf_isString(sexp) != 0 }
}

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
    // TODO: return a tagged pointer to represent an error
    if !is_string(x) {
        return R_NilValue;
    }

    let len = Rf_xlength(x);
    for i in 0..len {
        let e = STRING_ELT(x, i);
        if e == libR_sys::R_NaString {
            r_eprint(format!("(missing value)\n"));
            continue;
        }
        let e_cstr = CStr::from_ptr(Rf_translateCharUTF8(e));
        r_eprint(format!("{:?}\n", e_cstr.to_str()));
    }
    x
}

// #[no_mangle]
// pub unsafe extern "C" fn unextendr_set_car(x: SEXP, y: SEXP) -> SEXP {
//     let c = libR_sys::Rf_cons(x, libR_sys::R_NilValue);
//     libR_sys::SET_TAG(c, y);
//     c
// }
