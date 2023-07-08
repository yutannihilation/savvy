use libR_sys::SEXP;

// Since SEXP is not a Send, this needs to be implemented as a different type than thiserror.
pub type UnwindProtectResult = std::result::Result<SEXP, SEXP>;

extern "C" {
    fn unwind_protect_impl(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
    ) -> SEXP;
}

pub unsafe fn unwind_protect<F>(f: F) -> UnwindProtectResult
where
    F: FnOnce() -> SEXP + Copy,
{
    unsafe extern "C" fn do_call<F>(data: *mut std::os::raw::c_void) -> SEXP
    where
        F: FnOnce() -> SEXP + Copy,
    {
        let data = data as *const ();
        let f: &F = &*(data as *const F);
        f()
    }

    let fun_ptr = do_call::<F> as *const ();
    let fun = std::mem::transmute(fun_ptr);
    let data = std::mem::transmute(&f as *const F);
    let res: SEXP = unwind_protect_impl(fun, data);

    if (res as usize & 1) == 1 {
        return Err(res);
    }

    Ok(res)
}
