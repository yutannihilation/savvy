use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
};

use savvy_ffi::{
    altrep::{
        R_altrep_data2, R_make_altinteger_class, R_set_altinteger_Elt_method,
        R_set_altrep_Coerce_method, R_set_altrep_Duplicate_method, R_set_altrep_Inspect_method,
        R_set_altrep_Length_method, R_set_altrep_data2, R_set_altvec_Dataptr_method,
        R_set_altvec_Dataptr_or_null_method,
    },
    R_NaInt, R_NilValue, R_xlen_t, Rboolean, Rboolean_FALSE, Rboolean_TRUE, Rf_coerceVector,
    Rf_duplicate, Rf_protect, Rf_unprotect, INTEGER, INTSXP, SEXP, SEXPTYPE,
};

use crate::{IntegerSexp, IntoExtPtrSexp};

pub trait AltInteger: Sized + IntoExtPtrSexp {
    /// Class name to identify the ALTREP class.
    const CLASS_NAME: &'static str;

    /// Package name to identify the ALTREP class.
    const PACKAGE_NAME: &'static str;

    /// If `true`, cache the materialized SEXP. This means any updates on the
    /// underlying data are no longer reflected after the first materialization.
    /// So, it is strongly recommended to set this to `true` only when the
    /// underlying data doesn't change.
    const CACHE_MATERIALIZED_SEXP: bool = true;

    /// Return the length of the data.
    fn length(&mut self) -> usize;

    /// Returns the value of `i`-th element. Note that, it seems R handles the
    /// out-of-bound check, so you don't need to implement it here.
    fn elt(&mut self, i: usize) -> i32;

    /// Returns the pointer to the underlying data.
    fn dataptr(&mut self) -> Option<*mut i32> {
        None
    }

    /// Copies the specified range of the data into a new memory. This is used
    /// when the ALTREP needs to be materialized.
    ///
    /// For example, you can use `copy_from_slice()` for more efficient copying
    /// of the values.
    fn copy_to(&mut self, new: &mut [i32], offset: usize) {
        // TODO: return error
        if offset + new.len() > self.length() {
            return;
        }

        for (i, v) in new.iter_mut().enumerate() {
            *v = self.elt(i + offset);
        }
    }

    /// What gets printed when `.Internal(inspect(x))` is used.
    fn inspect(&mut self) {
        crate::io::r_print(&format!("({})", Self::CLASS_NAME), false);
    }

    /// Converts the struct into an ALTREP object
    fn into_altrep(self) -> crate::Result<SEXP> {
        super::create_altrep_instance(self, Self::CLASS_NAME, Self::CACHE_MATERIALIZED_SEXP)
    }

    /// Extracts the reference (`&T`) of the underlying data
    fn try_from_altrep_ref(x: &IntegerSexp) -> crate::Result<&Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_ref_from_altrep(&x.0)
    }

    /// Extracts the mutable reference (`&mut T`) of the underlying data
    fn try_from_altrep_mut(x: &mut IntegerSexp) -> crate::Result<&mut Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_mut_from_altrep(&mut x.0)
    }

    /// Takes the underlying data. After this operation, the external pointer is
    /// replaced with a null pointer.
    fn try_from_altrep(x: IntegerSexp) -> crate::Result<Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_from_altrep(x.0)
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn register_altinteger_class<T: AltInteger>(
    dll_info: *mut crate::ffi::DllInfo,
) -> crate::error::Result<()> {
    let class_name = CString::new(T::CLASS_NAME).unwrap_or_default();
    let package_name = CString::new(T::PACKAGE_NAME).unwrap_or_default();
    let class_t =
        unsafe { R_make_altinteger_class(class_name.as_ptr(), package_name.as_ptr(), dll_info) };

    #[allow(clippy::mut_from_ref)]
    #[inline]
    fn get_materialized_sexp<T: AltInteger>(x: &mut SEXP, allow_alocate: bool) -> Option<SEXP> {
        // If the strategy is to use cache the materialized SEXP, check if
        // there's already a cached one, and use it if it's available.
        if T::CACHE_MATERIALIZED_SEXP {
            let data = unsafe { R_altrep_data2(*x) };
            if unsafe { data != R_NilValue } {
                return Some(data);
            }
        }

        // If the allocation is unpreferable, give up here.
        if !allow_alocate {
            return None;
        }

        let self_: &mut T = match super::extract_mut_from_altrep(x) {
            Ok(self_) => self_,
            Err(_) => return None,
        };

        let len = self_.length();

        let new = crate::alloc_vector(INTSXP, len).unwrap();
        unsafe { Rf_protect(new) };

        self_.copy_to(
            unsafe { std::slice::from_raw_parts_mut(INTEGER(new), len) },
            0,
        );

        if T::CACHE_MATERIALIZED_SEXP {
            // Cache the materialized data in data2.
            unsafe { R_set_altrep_data2(*x, new) };
        }

        // new doesn't need protection because it's used as long as this ALTREP exists.
        unsafe { Rf_unprotect(1) };

        Some(new)
    }

    unsafe extern "C" fn altrep_duplicate<T: AltInteger>(
        mut x: SEXP,
        _deep_copy: Rboolean,
    ) -> SEXP {
        let materialized = get_materialized_sexp::<T>(&mut x, true).expect("Must have result");
        unsafe { Rf_duplicate(materialized) }
    }

    unsafe extern "C" fn altrep_coerce<T: AltInteger>(mut x: SEXP, sexp_type: SEXPTYPE) -> SEXP {
        let materialized = get_materialized_sexp::<T>(&mut x, true).expect("Must have result");
        unsafe { Rf_coerceVector(materialized, sexp_type) }
    }

    fn altvec_dataptr_inner<T: AltInteger>(mut x: SEXP, allow_allocate: bool) -> *mut c_void {
        let self_: &mut T = match super::extract_mut_from_altrep(&mut x) {
            Ok(self_) => self_,
            Err(_) => return unsafe { R_NilValue },
        };

        if T::CACHE_MATERIALIZED_SEXP {
            // If the strategy is to use the cached SEXP, do not call dataptr
            match get_materialized_sexp::<T>(&mut x, allow_allocate) {
                Some(materialized) => unsafe { INTEGER(materialized) as _ },
                // Returning C NULL (not R NULL!) is the convention
                None => std::ptr::null_mut(),
            }
        } else {
            match self_.dataptr() {
                Some(ptr) => ptr as _,
                None => std::ptr::null_mut(),
            }
        }
    }

    unsafe extern "C" fn altvec_dataptr<T: AltInteger>(
        x: SEXP,
        _writable: Rboolean,
    ) -> *mut c_void {
        altvec_dataptr_inner::<T>(x, true)
    }

    unsafe extern "C" fn altvec_dataptr_or_null<T: AltInteger>(x: SEXP) -> *const c_void {
        altvec_dataptr_inner::<T>(x, false)
    }

    unsafe extern "C" fn altrep_length<T: AltInteger>(mut x: SEXP) -> R_xlen_t {
        let self_: &mut T = match super::extract_mut_from_altrep(&mut x) {
            Ok(self_) => self_,
            Err(_) => return 0,
        };
        self_.length() as _
    }

    unsafe extern "C" fn altrep_inspect<T: AltInteger>(
        mut x: SEXP,
        _: c_int,
        _: c_int,
        _: c_int,
        _: Option<unsafe extern "C" fn(SEXP, c_int, c_int, c_int)>,
    ) -> Rboolean {
        let self_: &mut T = match super::extract_mut_from_altrep(&mut x) {
            Ok(self_) => self_,
            Err(_) => return Rboolean_FALSE,
        };
        self_.inspect();

        Rboolean_TRUE
    }

    unsafe extern "C" fn altinteger_elt<T: AltInteger>(mut x: SEXP, i: R_xlen_t) -> c_int {
        let self_: &mut T = match super::extract_mut_from_altrep(&mut x) {
            Ok(self_) => self_,
            Err(_) => return unsafe { R_NaInt },
        };
        self_.elt(i as _) as _
    }

    unsafe {
        R_set_altrep_Length_method(class_t, Some(altrep_length::<T>));
        R_set_altrep_Inspect_method(class_t, Some(altrep_inspect::<T>));
        R_set_altrep_Duplicate_method(class_t, Some(altrep_duplicate::<T>));
        R_set_altrep_Coerce_method(class_t, Some(altrep_coerce::<T>));
        R_set_altvec_Dataptr_method(class_t, Some(altvec_dataptr::<T>));
        R_set_altvec_Dataptr_or_null_method(class_t, Some(altvec_dataptr_or_null::<T>));
        R_set_altinteger_Elt_method(class_t, Some(altinteger_elt::<T>));
    }

    super::register_altrep_class(T::CLASS_NAME, class_t)?;
    Ok(())
}
