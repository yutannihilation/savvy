use savvy_ffi::SEXP;

extern "C" {
    fn unwind_protect_impl(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
    ) -> SEXP;
}

/// # Safety
///
/// This function wraps around `R_UnwindProtect()` API, which is very unsafe in
/// its nature. So, please use this with care.
pub unsafe fn unwind_protect<F>(f: F) -> crate::error::Result<SEXP>
where
    F: FnOnce() -> SEXP + Copy,
{
    unsafe {
        unsafe extern "C" fn do_call<F>(data: *mut std::os::raw::c_void) -> SEXP
        where
            F: FnOnce() -> SEXP + Copy,
        {
            unsafe {
                let data = data as *const ();
                let f: &F = &*(data as *const F);
                f()
            }
        }

        let fun_ptr = do_call::<F> as *const ();
        let fun = std::mem::transmute(fun_ptr);
        let data = std::mem::transmute(&f as *const F);
        let res: SEXP = unwind_protect_impl(fun, data);

        if (res as usize & 1) == 1 {
            return Err(crate::error::Error::Aborted(res));
        }

        Ok(res)
    }
}
