use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
};

use savvy_ffi::{
    altrep::{
        R_altrep_data2, R_make_altinteger_class, R_set_altinteger_Elt_method,
        R_set_altinteger_Max_method, R_set_altinteger_Min_method, R_set_altinteger_Sum_method,
        R_set_altrep_Coerce_method, R_set_altrep_Duplicate_method, R_set_altrep_Inspect_method,
        R_set_altrep_Length_method, R_set_altrep_data2, R_set_altvec_Dataptr_method,
        R_set_altvec_Dataptr_or_null_method,
    },
    R_NaInt, R_NilValue, R_xlen_t, Rboolean, Rboolean_FALSE, Rboolean_TRUE, Rf_ScalarInteger,
    Rf_ScalarReal, Rf_coerceVector, Rf_duplicate, Rf_protect, Rf_unprotect, Rf_xlength, INTEGER,
    INTEGER_ELT, INTSXP, SEXP, SEXPTYPE,
};

use crate::{IntegerSexp, IntoExtPtrSexp, NotAvailableValue};

pub trait AltInteger: Sized + IntoExtPtrSexp {
    /// Class name to identify the ALTREP class.
    const CLASS_NAME: &'static str;

    /// Package name to identify the ALTREP class.
    const PACKAGE_NAME: &'static str;

    /// Returns the length of the data.
    fn length(&mut self) -> usize;

    /// Returns the value of `i`-th element. Note that, it seems R handles the
    /// out-of-bound check, so you don't need to implement it here.
    fn elt(&mut self, i: usize) -> i32;

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

    /// Return the sum of all values.
    ///
    /// This returns `f64` because the sum of `i32` might be beyond the range of `i32`.
    fn sum(&mut self, na_rm: bool) -> Option<f64> {
        let mut result = 0.0;
        for i in 0..self.length() {
            let x = self.elt(i);
            if x.is_na() {
                if na_rm {
                    continue;
                } else {
                    return None;
                }
            } else {
                result += x as f64;
            }
        }

        Some(result)
    }

    /// Return the minimum value.
    ///
    /// This returns `f64` because `min()` of an empty vector is `Inf`. Note
    /// that it's this function's responsibility to handle such cases:
    ///
    /// - `min(integer(0L))`
    /// - `min(c(NA_integer_), na.rm = TRUE)`
    fn min(&mut self, na_rm: bool) -> Option<f64> {
        let mut result = f64::INFINITY;
        for i in 0..self.length() {
            let x = self.elt(i);
            if x.is_na() {
                if na_rm {
                    continue;
                } else {
                    return None;
                }
            } else {
                result = f64::min(result, x as f64);
            }
        }

        Some(result)
    }

    /// Return the maximum value.
    ///
    /// This returns `f64` because `max()` of an empty vector is `-Inf`. Note
    /// that it's this function's responsibility to handle such cases:
    ///
    /// - `max(integer(0L))`
    /// - `max(c(NA_integer_), na.rm = TRUE)`
    fn max(&mut self, na_rm: bool) -> Option<f64> {
        let mut result = f64::NEG_INFINITY;
        for i in 0..self.length() {
            let x = self.elt(i);
            if x.is_na() {
                if na_rm {
                    continue;
                } else {
                    return None;
                }
            } else {
                result = f64::max(result, x as f64);
            }
        }

        Some(result)
    }

    /// What gets printed when `.Internal(inspect(x))` is used.
    fn inspect(&mut self, is_materialized: bool) {
        crate::io::r_print(
            &format!("{} (materialized: {is_materialized})\n", Self::CLASS_NAME),
            false,
        );
    }

    /// Converts the struct into an ALTREP object.
    fn into_altrep(self) -> crate::Result<crate::Sexp> {
        super::create_altrep_instance(self, Self::CLASS_NAME).map(crate::Sexp)
    }

    /// Extracts the reference (`&T`) of the underlying data.
    fn try_from_altrep_ref(x: &IntegerSexp) -> crate::Result<&Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_ref_from_altrep(&x.0)
    }

    /// Extracts the mutable reference (`&mut T`) of the underlying data.
    fn try_from_altrep_mut(
        x: &mut IntegerSexp,
        invalidate_cache: bool,
    ) -> crate::Result<&mut Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        if invalidate_cache {
            unsafe { R_set_altrep_data2(x.0, R_NilValue) }
        }
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
    fn get_materialized_sexp<T: AltInteger>(x: &mut SEXP, allow_allocate: bool) -> Option<SEXP> {
        let data = unsafe { R_altrep_data2(*x) };
        if unsafe { data != R_NilValue } {
            return Some(data);
        }

        // If the allocation is unpreferable, give up here.
        if !allow_allocate {
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

        crate::log::debug!("A {} object is materialized", T::CLASS_NAME);

        // Cache the materialized data in data2.
        unsafe { R_set_altrep_data2(*x, new) };

        // new doesn't need protection because it's used as long as this ALTREP exists.
        unsafe { Rf_unprotect(1) };

        Some(new)
    }

    unsafe extern "C" fn altrep_duplicate<T: AltInteger>(
        mut x: SEXP,
        _deep_copy: Rboolean,
    ) -> SEXP {
        crate::log::trace!("A {} object is duplicated", T::CLASS_NAME);

        let materialized = get_materialized_sexp::<T>(&mut x, true).expect("Must have result");
        unsafe { Rf_duplicate(materialized) }
    }

    unsafe extern "C" fn altrep_coerce<T: AltInteger>(mut x: SEXP, sexp_type: SEXPTYPE) -> SEXP {
        crate::log::trace!("A {} object is coerced", T::CLASS_NAME);

        let materialized = get_materialized_sexp::<T>(&mut x, true).expect("Must have result");
        unsafe { Rf_coerceVector(materialized, sexp_type) }
    }

    fn altvec_dataptr_inner<T: AltInteger>(mut x: SEXP, allow_allocate: bool) -> *mut c_void {
        match get_materialized_sexp::<T>(&mut x, allow_allocate) {
            Some(materialized) => unsafe { INTEGER(materialized) as _ },
            // Returning C NULL (not R NULL!) is the convention
            None => std::ptr::null_mut(),
        }
    }

    unsafe extern "C" fn altvec_dataptr<T: AltInteger>(
        x: SEXP,
        _writable: Rboolean,
    ) -> *mut c_void {
        crate::log::trace!("DATAPTR({}) is called", T::CLASS_NAME);

        altvec_dataptr_inner::<T>(x, true)
    }

    unsafe extern "C" fn altvec_dataptr_or_null<T: AltInteger>(x: SEXP) -> *const c_void {
        crate::log::trace!("DATAPTR_OR_NULL({}) is called", T::CLASS_NAME);

        altvec_dataptr_inner::<T>(x, false)
    }

    unsafe extern "C" fn altrep_length<T: AltInteger>(mut x: SEXP) -> R_xlen_t {
        if let Some(materialized) = get_materialized_sexp::<T>(&mut x, false) {
            unsafe { Rf_xlength(materialized) }
        } else {
            match super::extract_mut_from_altrep::<T>(&mut x) {
                Ok(self_) => self_.length() as _,
                Err(_) => 0,
            }
        }
    }

    unsafe extern "C" fn altrep_inspect<T: AltInteger>(
        mut x: SEXP,
        _: c_int,
        _: c_int,
        _: c_int,
        _: Option<unsafe extern "C" fn(SEXP, c_int, c_int, c_int)>,
    ) -> Rboolean {
        let is_materialized = unsafe { R_altrep_data2(x) != R_NilValue };
        match super::extract_mut_from_altrep::<T>(&mut x) {
            Ok(self_) => {
                self_.inspect(is_materialized);
                Rboolean_TRUE
            }
            Err(_) => Rboolean_FALSE,
        }
    }

    unsafe extern "C" fn altinteger_elt<T: AltInteger>(mut x: SEXP, i: R_xlen_t) -> c_int {
        crate::log::trace!("INTEGER_ELT({}, {i}) is called", T::CLASS_NAME);

        if let Some(materialized) = get_materialized_sexp::<T>(&mut x, false) {
            unsafe { INTEGER_ELT(materialized, i) }
        } else {
            match super::extract_mut_from_altrep::<T>(&mut x) {
                Ok(self_) => self_.elt(i as _) as _,
                Err(_) => unsafe { R_NaInt },
            }
        }
    }

    unsafe extern "C" fn altinteger_sum<T: AltInteger>(mut x: SEXP, na_rm: Rboolean) -> SEXP {
        crate::log::trace!("ALTINTEGER_SUM({}) is called", T::CLASS_NAME);

        let na_rm = na_rm == Rboolean_TRUE;

        let sum: f64 = if let Some(materialized) = get_materialized_sexp::<T>(&mut x, false) {
            let mut result = 0.0;
            let slice = unsafe {
                std::slice::from_raw_parts(
                    INTEGER(materialized) as *mut i32,
                    Rf_xlength(materialized) as _,
                )
            };
            for &x in slice {
                if x.is_na() {
                    if na_rm {
                        continue;
                    } else {
                        return unsafe { Rf_ScalarInteger(i32::na()) };
                    }
                } else {
                    result = result + x as f64;
                }
            }
            result
        } else {
            match super::extract_mut_from_altrep::<T>(&mut x) {
                Ok(self_) => match self_.sum(na_rm) {
                    Some(sum) => sum,
                    None => return unsafe { Rf_ScalarInteger(i32::na()) },
                },
                Err(_) => {
                    // TODO: should be error, but there's no way to throw error safely from here.
                    return unsafe { Rf_ScalarInteger(i32::na()) };
                }
            }
        };

        maybe_int(sum)
    }

    unsafe extern "C" fn altinteger_min<T: AltInteger>(mut x: SEXP, na_rm: Rboolean) -> SEXP {
        crate::log::trace!("ALTINTEGER_MIN({}) is called", T::CLASS_NAME);

        let na_rm = na_rm == Rboolean_TRUE;

        // min(integer()) returns Inf
        let len = unsafe { altrep_length::<T>(x) };
        if len == 0 {
            return unsafe { Rf_ScalarReal(f64::INFINITY) };
        }

        let result = if let Some(materialized) = get_materialized_sexp::<T>(&mut x, false) {
            let mut result = f64::INFINITY;
            let slice =
                unsafe { std::slice::from_raw_parts(INTEGER(materialized) as *mut i32, len as _) };
            for &x in slice {
                if x.is_na() {
                    if na_rm {
                        continue;
                    } else {
                        return unsafe { Rf_ScalarInteger(i32::na()) };
                    }
                } else {
                    result = f64::min(result, x as f64);
                }
            }
            result
        } else {
            match super::extract_mut_from_altrep::<T>(&mut x) {
                Ok(self_) => match self_.min(na_rm) {
                    Some(min) => min,
                    None => return unsafe { Rf_ScalarInteger(i32::na()) },
                },
                Err(_) => {
                    // TODO: probably this should be error (or panic?),
                    // but there's no safe way to throw error from here.
                    return unsafe { Rf_ScalarInteger(i32::na()) };
                }
            }
        };

        maybe_int(result)
    }

    unsafe extern "C" fn altinteger_max<T: AltInteger>(mut x: SEXP, na_rm: Rboolean) -> SEXP {
        crate::log::trace!("ALTINTEGER_MAX({}) is called", T::CLASS_NAME);

        let na_rm = na_rm == Rboolean_TRUE;

        // max(integer()) returns -Inf
        let len = unsafe { altrep_length::<T>(x) };
        if len == 0 {
            return unsafe { Rf_ScalarReal(f64::NEG_INFINITY) };
        }

        let result = if let Some(materialized) = get_materialized_sexp::<T>(&mut x, false) {
            let mut result = f64::NEG_INFINITY;
            let slice =
                unsafe { std::slice::from_raw_parts(INTEGER(materialized) as *mut i32, len as _) };
            for &x in slice {
                if x.is_na() {
                    if na_rm {
                        continue;
                    } else {
                        return unsafe { Rf_ScalarInteger(i32::na()) };
                    }
                } else {
                    result = f64::max(result, x as f64);
                }
            }
            result
        } else {
            match super::extract_mut_from_altrep::<T>(&mut x) {
                Ok(self_) => match self_.max(na_rm) {
                    Some(max) => max,
                    None => return unsafe { Rf_ScalarInteger(i32::na()) },
                },
                Err(_) => {
                    // TODO: probably this should be error (or panic?),
                    // but there's no safe way to throw error from here.
                    return unsafe { Rf_ScalarInteger(i32::na()) };
                }
            }
        };

        maybe_int(result)
    }

    unsafe {
        R_set_altrep_Length_method(class_t, Some(altrep_length::<T>));
        R_set_altrep_Inspect_method(class_t, Some(altrep_inspect::<T>));
        R_set_altrep_Duplicate_method(class_t, Some(altrep_duplicate::<T>));
        R_set_altrep_Coerce_method(class_t, Some(altrep_coerce::<T>));
        R_set_altvec_Dataptr_method(class_t, Some(altvec_dataptr::<T>));
        R_set_altvec_Dataptr_or_null_method(class_t, Some(altvec_dataptr_or_null::<T>));
        R_set_altinteger_Elt_method(class_t, Some(altinteger_elt::<T>));
        R_set_altinteger_Sum_method(class_t, Some(altinteger_sum::<T>));
        R_set_altinteger_Min_method(class_t, Some(altinteger_min::<T>));
        R_set_altinteger_Max_method(class_t, Some(altinteger_max::<T>));
    }

    super::register_altrep_class(T::CLASS_NAME, class_t)?;
    Ok(())
}

fn maybe_int(x: f64) -> SEXP {
    // Note: Why we add +1 to i32::MIN? This is a bit tricky.
    // i32::MIN is R_NaInt, so it cannot be included in the range.
    if (i32::MIN + 1) as f64 <= x && x <= i32::MAX as f64 {
        unsafe { Rf_ScalarInteger(x.to_int_unchecked()) }
    } else {
        unsafe { Rf_ScalarReal(x) }
    }
}
