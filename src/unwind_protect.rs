use std::os::raw::c_void;

use savvy_ffi::SEXP;

extern "C" {
    fn unwind_protect_impl(
        fun: Option<unsafe extern "C" fn(data: *mut c_void) -> SEXP>,
        data: *mut c_void,
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
        unsafe extern "C" fn do_call<F>(data: *mut c_void) -> SEXP
        where
            F: FnOnce() -> SEXP + Copy,
        {
            unsafe {
                let data = data as *const ();
                let f: &F = &*(data as *const F);
                f()
            }
        }

        let do_call_ptr = std::mem::transmute::<
            *const (),
            Option<unsafe extern "C" fn(*mut c_void) -> SEXP>,
        >(do_call::<F> as *const ());
        let actual_fn_ptr = std::mem::transmute::<*const F, *mut c_void>(&f as *const F);
        let res: SEXP = unwind_protect_impl(do_call_ptr, actual_fn_ptr);

        if (res as usize & 1) == 1 {
            return Err(crate::error::Error::Aborted(res));
        }

        Ok(res)
    }
}
