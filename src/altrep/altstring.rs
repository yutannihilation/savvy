use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
};

use savvy_ffi::{
    altrep::{
        R_altrep_data2, R_make_altstring_class, R_set_altrep_Coerce_method,
        R_set_altrep_Duplicate_method, R_set_altrep_Inspect_method, R_set_altrep_Length_method,
        R_set_altrep_data2, R_set_altstring_Elt_method, R_set_altvec_Dataptr_method,
        R_set_altvec_Dataptr_or_null_method,
    },
    R_NaString, R_NilValue, R_xlen_t, Rboolean, Rboolean_TRUE, Rf_coerceVector, Rf_duplicate,
    Rf_protect, Rf_unprotect, SET_STRING_ELT, SEXP, SEXPTYPE, STRING_PTR, STRING_PTR_RO, STRSXP,
};

use crate::IntoExtPtrSexp;

pub trait AltString: Sized + IntoExtPtrSexp {
    /// Class name to identify the ALTREP class.
    const CLASS_NAME: &'static str;

    /// Package name to identify the ALTREP class.
    const PACKAGE_NAME: &'static str;

    fn into_altrep(self) -> crate::Result<SEXP> {
        super::create_altrep_instance(self, Self::CLASS_NAME)
    }

    /// What gets printed when `.Internal(inspect(x))` is used.
    fn inspect(&mut self) {
        crate::io::r_print(&format!("({})", Self::CLASS_NAME), false);
    }

    /// Return the length of the data.
    fn length(&mut self) -> usize;

    /// Returns the value of `i`-th element. Note that, it seems R handles the
    /// out-of-bound check, so you don't need to implement it here.
    fn elt(&mut self, i: usize) -> &str;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn register_altstring_class<T: AltString>(
    dll_info: *mut crate::ffi::DllInfo,
) -> crate::error::Result<()> {
    let class_name = CString::new(T::CLASS_NAME).unwrap_or_default();
    let package_name = CString::new(T::PACKAGE_NAME).unwrap_or_default();
    let class_t =
        unsafe { R_make_altstring_class(class_name.as_ptr(), package_name.as_ptr(), dll_info) };

    #[allow(clippy::mut_from_ref)]
    #[inline]
    fn materialize<T: AltString>(x: &SEXP) -> SEXP {
        let data = unsafe { R_altrep_data2(*x) };
        if unsafe { data != R_NilValue } {
            return data;
        }

        let self_: &mut T = super::extract_self_from_altrep(x);

        let len = self_.length();
        let new = crate::alloc_vector(STRSXP, len).unwrap();

        unsafe { Rf_protect(new) };

        for i in 0..len {
            unsafe {
                SET_STRING_ELT(
                    new,
                    i as _,
                    crate::sexp::utils::str_to_charsxp(self_.elt(i)).unwrap_or(R_NaString),
                )
            };
        }

        // Cache the materialized data in data2.
        //
        // Note that, for example arrow stores it in `CAR()` of data2, but this
        // implementation naively uses data2. Probably that should be clever
        // because data2 can be used for other purposes.
        unsafe { R_set_altrep_data2(*x, new) };

        // new doesn't need protection because it's used as long as this ALTREP exists.
        unsafe { Rf_unprotect(1) };

        new
    }

    unsafe extern "C" fn altrep_duplicate<T: AltString>(x: SEXP, _deep_copy: Rboolean) -> SEXP {
        let materialized = materialize::<T>(&x);

        // let attrs = unsafe { Rf_protect(Rf_duplicate(ATTRIB(x))) };
        // unsafe { SET_ATTRIB(materialized, attrs) };

        unsafe { Rf_duplicate(materialized) }
    }

    unsafe extern "C" fn altrep_coerce<T: AltString>(x: SEXP, sexp_type: SEXPTYPE) -> SEXP {
        let materialized = materialize::<T>(&x);
        unsafe { Rf_coerceVector(materialized, sexp_type) }
    }

    unsafe extern "C" fn altvec_dataptr<T: AltString>(x: SEXP, _writable: Rboolean) -> *mut c_void {
        let materialized = materialize::<T>(&x);
        unsafe { STRING_PTR(materialized) as _ }
    }

    unsafe extern "C" fn altvec_dataptr_or_null<T: AltString>(x: SEXP) -> *const c_void {
        let materialized = materialize::<T>(&x);
        unsafe { STRING_PTR_RO(materialized) as _ }
    }

    unsafe extern "C" fn altrep_length<T: AltString>(x: SEXP) -> R_xlen_t {
        let self_: &mut T = super::extract_self_from_altrep(&x);
        self_.length() as _
    }

    unsafe extern "C" fn altrep_inspect<T: AltString>(
        x: SEXP,
        _: c_int,
        _: c_int,
        _: c_int,
        _: Option<unsafe extern "C" fn(SEXP, c_int, c_int, c_int)>,
    ) -> Rboolean {
        let self_: &mut T = super::extract_self_from_altrep(&x);
        self_.inspect();

        Rboolean_TRUE
    }

    unsafe extern "C" fn altstring_elt<T: AltString>(x: SEXP, i: R_xlen_t) -> SEXP {
        let self_: &mut T = super::extract_self_from_altrep(&x);
        unsafe { crate::sexp::utils::str_to_charsxp(self_.elt(i as _)).unwrap_or(R_NaString) }
    }

    unsafe {
        R_set_altrep_Length_method(class_t, Some(altrep_length::<T>));
        R_set_altrep_Inspect_method(class_t, Some(altrep_inspect::<T>));
        R_set_altrep_Duplicate_method(class_t, Some(altrep_duplicate::<T>));
        R_set_altrep_Coerce_method(class_t, Some(altrep_coerce::<T>));
        R_set_altvec_Dataptr_method(class_t, Some(altvec_dataptr::<T>));
        R_set_altvec_Dataptr_or_null_method(class_t, Some(altvec_dataptr_or_null::<T>));
        R_set_altstring_Elt_method(class_t, Some(altstring_elt::<T>));
    }

    super::register_altrep_class(T::CLASS_NAME, class_t)?;
    Ok(())
}
