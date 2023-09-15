use libR_sys::{
    R_ClearExternalPtr, R_ExternalPtrAddr, R_MakeExternalPtr, R_NilValue, R_RegisterCFinalizerEx,
    Rf_protect, Rf_unprotect, SEXP,
};

// Some notes about the design.
//
// 1. conversion from Rust struct into SEXP
//
// The result `EXTPTRSXP` is returned as unprotected because the conversion
// happens very before returning the result to C's side. This means there should
// be no more call to R API until it's finally passed to the R session.
//
// 2. conversion from SEXP into Rust struct
//
// This conversion is handled in macro because it's basically just
// `R_ExternalPtrAddr(x) as *mut T` if we can optimistically assume the user never
// supply a wrong input. This assumption should be ensured on R's side.

pub trait IntoExtPtrSxp: Sized {
    // Note: I can add two more arguments here just as cpp11 does
    // (https://github.com/r-lib/cpp11/blob/500f642b4ea132ec8c168fc70a28e81e9510ece3/inst/include/cpp11/external_pointer.hpp#L58)
    //
    // 1. use_deleter
    //
    // It's not always the cleanup process should be called automatically. If
    // this is false, R_RegisterCFinalizerEx() is not called.
    //
    // 2. finalize_on_exit
    //
    // R_RegisterCFinalizerEx() has `onexit` argument. On R-exts, it is
    // described as below:
    //
    // >  the onexit argument of the extended forms can be used to ask that the
    // >  finalizer be run during a normal shutdown of the R session.
    //
    // I'm not immediately sure about the pros and cons, but I bet it's good to
    // enable this by default.
    fn into_external_pointer(self) -> SEXP {
        let boxed = Box::new(self);
        let ptr = Box::into_raw(boxed);

        unsafe extern "C" fn finalizer<T>(x: SEXP) {
            // bring back the ownership to Rust's side so that Rust will drop
            // after this block ends.
            let _ = Box::from_raw(R_ExternalPtrAddr(x) as *mut T);

            R_ClearExternalPtr(x);
        }

        unsafe {
            let external_pointer = Rf_protect(R_MakeExternalPtr(
                ptr as *mut std::os::raw::c_void,
                R_NilValue,
                R_NilValue,
            ));

            // Use R_RegisterCFinalizerEx(..., TRUE) instead of
            // R_RegisterCFinalizer() in order to make the cleanup happen during
            // a shutdown of the R session as well.
            R_RegisterCFinalizerEx(external_pointer, Some(finalizer::<Self>), 1);

            Rf_unprotect(1);

            external_pointer
        }
    }
}

pub unsafe fn get_external_pointer_addr(x: SEXP) -> *mut std::os::raw::c_void {
    R_ExternalPtrAddr(x)
}
