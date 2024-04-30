use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
};

use savvy_ffi::{
    altrep::{
        R_altrep_data2, R_make_altlogical_class, R_set_altlogical_Elt_method,
        R_set_altrep_Coerce_method, R_set_altrep_Duplicate_method, R_set_altrep_Inspect_method,
        R_set_altrep_Length_method, R_set_altrep_data2, R_set_altvec_Dataptr_method,
        R_set_altvec_Dataptr_or_null_method,
    },
    R_NaInt, R_NilValue, R_xlen_t, Rboolean, Rboolean_FALSE, Rboolean_TRUE, Rf_coerceVector,
    Rf_duplicate, Rf_protect, Rf_unprotect, LGLSXP, LOGICAL, LOGICAL_RO, SEXP, SEXPTYPE,
};

use crate::{IntoExtPtrSexp, LogicalSexp};

pub trait AltLogical: Sized + IntoExtPtrSexp {
    /// Class name to identify the ALTREP class.
    const CLASS_NAME: &'static str;

    /// Package name to identify the ALTREP class.
    const PACKAGE_NAME: &'static str;

    /// Converts the struct into an ALTREP object
    fn into_altrep(self) -> crate::Result<SEXP> {
        super::create_altrep_instance(self, Self::CLASS_NAME)
    }

    /// Extracts the reference (`&T`) of the underlying data
    fn try_from_altrep_ref(x: &LogicalSexp) -> crate::Result<&Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_ref_from_altrep(&x.0)
    }

    /// Extracts the mutable reference (`&mut T`) of the underlying data
    fn try_from_altrep_mut(x: &mut LogicalSexp) -> crate::Result<&mut Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_mut_from_altrep(&mut x.0)
    }

    /// Takes the underlying data. After this operation, the external pointer is
    /// replaced with a null pointer.
    fn try_from_altrep(x: LogicalSexp) -> crate::Result<Self> {
        super::assert_altrep_class(x.0, Self::CLASS_NAME)?;
        super::extract_from_altrep(x.0)
    }

    /// Copies all the data into a new memory. This is used when the ALTREP
    /// needs to be materialized.
    ///
    /// For example, you can use `copy_from_slice()` for more efficient copying
    /// of the values.
    fn copy_data(&mut self, new: &mut [i32]) {
        for (i, v) in new.iter_mut().enumerate() {
            *v = self.elt(i) as _;
        }
    }

    /// What gets printed when `.Internal(inspect(x))` is used.
    fn inspect(&mut self) {
        crate::io::r_print(&format!("({})", Self::CLASS_NAME), false);
    }

    /// Return the length of the data.
    fn length(&mut self) -> usize;

    /// Returns the value of `i`-th element. Note that, it seems R handles the
    /// out-of-bound check, so you don't need to implement it here.
    fn elt(&mut self, i: usize) -> bool;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn register_altlogical_class<T: AltLogical>(
    dll_info: *mut crate::ffi::DllInfo,
) -> crate::error::Result<()> {
    let class_name = CString::new(T::CLASS_NAME).unwrap_or_default();
    let package_name = CString::new(T::PACKAGE_NAME).unwrap_or_default();
    let class_t =
        unsafe { R_make_altlogical_class(class_name.as_ptr(), package_name.as_ptr(), dll_info) };

    #[allow(clippy::mut_from_ref)]
    #[inline]
    fn materialize<T: AltLogical>(x: &mut SEXP) -> SEXP {
        let data = unsafe { R_altrep_data2(*x) };
        if unsafe { data != R_NilValue } {
            return data;
        }

        let self_: &mut T = match super::extract_mut_from_altrep(x) {
            Ok(self_) => self_,
            Err(_) => return unsafe { R_NilValue },
        };

        let len = self_.length();
        let new = crate::alloc_vector(LGLSXP, len).unwrap();

        unsafe { Rf_protect(new) };

        let dst = unsafe { std::slice::from_raw_parts_mut(LOGICAL(new), len) };

        self_.copy_data(dst);

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

    unsafe extern "C" fn altrep_duplicate<T: AltLogical>(
        mut x: SEXP,
        _deep_copy: Rboolean,
    ) -> SEXP {
        let materialized = materialize::<T>(&mut x);

        // let attrs = unsafe { Rf_protect(Rf_duplicate(ATTRIB(x))) };
        // unsafe { SET_ATTRIB(materialized, attrs) };

        unsafe { Rf_duplicate(materialized) }
    }

    unsafe extern "C" fn altrep_coerce<T: AltLogical>(mut x: SEXP, sexp_type: SEXPTYPE) -> SEXP {
        let materialized = materialize::<T>(&mut x);
        unsafe { Rf_coerceVector(materialized, sexp_type) }
    }

    unsafe extern "C" fn altvec_dataptr<T: AltLogical>(
        mut x: SEXP,
        _writable: Rboolean,
    ) -> *mut c_void {
        let materialized = materialize::<T>(&mut x);
        unsafe { LOGICAL(materialized) as _ }
    }

    unsafe extern "C" fn altvec_dataptr_or_null<T: AltLogical>(mut x: SEXP) -> *const c_void {
        let materialized = materialize::<T>(&mut x);
        unsafe { LOGICAL_RO(materialized) as _ }
    }

    unsafe extern "C" fn altrep_length<T: AltLogical>(mut x: SEXP) -> R_xlen_t {
        let self_: &mut T = match super::extract_mut_from_altrep(&mut x) {
            Ok(self_) => self_,
            Err(_) => return 0,
        };
        self_.length() as _
    }

    unsafe extern "C" fn altrep_inspect<T: AltLogical>(
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

    unsafe extern "C" fn altlogical_elt<T: AltLogical>(mut x: SEXP, i: R_xlen_t) -> c_int {
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
        R_set_altlogical_Elt_method(class_t, Some(altlogical_elt::<T>));
    }

    super::register_altrep_class(T::CLASS_NAME, class_t)?;
    Ok(())
}
