use libR_sys::SEXP;

#[no_mangle]
pub extern "C" fn unextendr_string(x: SEXP) -> SEXP {
    x
}
