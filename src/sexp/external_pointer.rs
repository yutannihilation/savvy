use savvy_ffi::{
    R_ClearExternalPtr, R_ExternalPtrAddr, R_MakeExternalPtr, R_NilValue, R_RegisterCFinalizerEx,
    Rf_protect, Rf_unprotect, SEXP,
};

use crate::Sexp;

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

pub trait IntoExtPtrSexp: Sized {
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
    fn into_external_pointer(self) -> Sexp {
        let boxed = Box::new(self);
        let ptr = Box::into_raw(boxed);

        unsafe extern "C" fn finalizer<T>(x: SEXP) {
            // bring back the ownership to Rust's side so that Rust will drop
            // after this block ends.
            let ptr = unsafe { R_ExternalPtrAddr(x) };

            // the pointer can be null (e.g. https://github.com/pola-rs/r-polars/issues/851)
            if !ptr.is_null() {
                let rust_obj = unsafe { Box::from_raw(ptr as *mut T) };
                drop(rust_obj);
            }

            unsafe { R_ClearExternalPtr(x) };
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

            Sexp(external_pointer)
        }
    }
}

/// A Wrapper of R_ExternalPtrAddr() to use in savvy-bindgen
///
/// ## Safety
/// This is intended to be used only in savvy-bindgen
pub unsafe fn get_external_pointer_addr(
    x: SEXP,
) -> crate::error::Result<*mut std::os::raw::c_void> {
    let ptr = unsafe { R_ExternalPtrAddr(x) };

    if ptr.is_null() {
        return Err(crate::error::Error::InvalidPointer);
    }

    Ok(ptr)
}

/// Takes the value of the external pointer and set the pointer to null.
///
/// ## Safety
/// This is intended to be used only in savvy-bindgen
pub unsafe fn take_external_pointer_value<T>(x: SEXP) -> crate::error::Result<T> {
    let ptr = unsafe { R_ExternalPtrAddr(x) };

    if !ptr.is_null() {
        let rust_obj = unsafe { Box::from_raw(ptr as *mut T) };

        // Set the pointer to null
        unsafe { R_ClearExternalPtr(x) };

        Ok(*rust_obj)
    } else {
        Err(crate::error::Error::InvalidPointer)
    }
}

/// An **external** external pointer.
///
/// This exists solely for casting a EXTPTRSXP into the underlying type.
pub struct ExternalPointerSexp(pub SEXP);

impl ExternalPointerSexp {
    pub fn inner(&self) -> SEXP {
        self.0
    }

    /// Cast the SEXP to a concrete type of pointer.
    ///
    /// # Safety
    ///
    /// This function is highly unsafe in that there's no mechanism to verify
    /// the destination type is the correct one.
    pub unsafe fn cast_unchecked<T>(&self) -> *const T {
        unsafe { savvy_ffi::R_ExternalPtrAddr(self.0) as _ }
    }

    /// Cast the SEXP to a concrete type of pointer.
    ///
    /// # Safety
    ///
    /// This function is highly unsafe in that there's no mechanism to verify
    /// the destination type is the correct one.
    pub unsafe fn cast_mut_unchecked<T>(&self) -> *mut T {
        unsafe { savvy_ffi::R_ExternalPtrAddr(self.0) as _ }
    }
}

impl TryFrom<Sexp> for ExternalPointerSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        // Return error if the SEXP is not an external pointer
        value.assert_external_pointer()?;

        Ok(Self(value.0))
    }
}
