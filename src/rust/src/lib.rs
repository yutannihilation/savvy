use libR_sys::{
    cetype_t_CE_UTF8, REprintf, Rf_allocVector, Rf_mkCharLenCE, Rprintf, SET_STRING_ELT, SEXP,
};
use std::ffi::CString;

mod error;
mod protect;
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

    let out = Rf_allocVector(libR_sys::STRSXP, x.len() as _);

    // // Do I need to protect here? Or, as this will be passed to R's side, it's not needed?
    // protect::PRESERVED_LIST.insert(out);

    for i in 0..x.len() {
        let e = &x[i];

        let e_upper = e.to_uppercase();

        // Rf_mkCharLenCE() probably allocates
        let r_str = Rf_mkCharLenCE(
            e_upper.as_ptr() as *const i8,
            e_upper.len() as i32,
            cetype_t_CE_UTF8,
        );

        SET_STRING_ELT(out, i as isize, r_str);
    }

    out
}

// #[no_mangle]
// pub unsafe extern "C" fn unextendr_set_car(x: SEXP, y: SEXP) -> SEXP {
//     let c = libR_sys::Rf_cons(x, libR_sys::R_NilValue);
//     libR_sys::SET_TAG(c, y);
//     c
// }
