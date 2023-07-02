use libR_sys::SEXP;

extern "C" {
    pub(crate) fn unwind_protect(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
    ) -> SEXP;
}
