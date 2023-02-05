use libR_sys::SEXP;

#[no_mangle]
pub extern "C" fn string(x: SEXP) -> SEXP {
    x
}
