use std::{collections::HashMap, ffi::CString, os::raw::c_void, sync::Mutex};

use once_cell::sync::OnceCell;
use savvy_ffi::{
    altrep::{
        R_altrep_class_t, R_altrep_data1, R_altrep_data2, R_make_altinteger_class, R_new_altrep,
        R_set_altinteger_Elt_method, R_set_altrep_Coerce_method, R_set_altrep_Duplicate_method,
        R_set_altrep_Inspect_method, R_set_altrep_Length_method, R_set_altrep_data2,
        R_set_altvec_Dataptr_method, R_set_altvec_Dataptr_or_null_method, MARK_NOT_MUTABLE,
    },
    R_NilValue, R_xlen_t, Rboolean, Rboolean_TRUE, Rf_coerceVector, Rf_duplicate, Rf_protect,
    Rf_unprotect, INTEGER, INTSXP, SEXP, SEXPTYPE,
};

use crate::{protect::local_protect, IntoExtPtrSexp};

static ALTREP_CLASS_CATALOGUE: OnceCell<Mutex<HashMap<&'static str, R_altrep_class_t>>> =
    OnceCell::new();

pub fn create_altrep_instance<T: 'static + AltInteger + IntoExtPtrSexp>(
    x: T,
) -> crate::Result<SEXP> {
    let sexp = x.into_external_pointer().0;
    local_protect(sexp);

    let catalogue_mutex = match ALTREP_CLASS_CATALOGUE.get() {
        Some(catalogue_mutex) => catalogue_mutex,
        None => return Err("ALTREP_CLASS_CATALOGUE is not initialized".into()),
    };
    let catalogue = match catalogue_mutex.lock() {
        Ok(catalogue) => catalogue,
        Err(e) => return Err(e.to_string().into()),
    };
    let class = match catalogue.get(T::CLASS_NAME) {
        Some(class) => class,
        None => return Err("Failed to get the ALTREP class".into()),
    };

    let altrep = unsafe { R_new_altrep(*class, sexp, R_NilValue) };
    local_protect(altrep);
    unsafe { MARK_NOT_MUTABLE(altrep) };

    Ok(altrep)
}

fn register_altrep_class(
    class_name: &'static str,
    class_t: R_altrep_class_t,
) -> crate::error::Result<()> {
    // There's no way to let global
    ALTREP_CLASS_CATALOGUE.get_or_init(|| Mutex::new(HashMap::new()));

    let catalogue_mutex = match ALTREP_CLASS_CATALOGUE.get() {
        Some(catalogue_mutex) => catalogue_mutex,
        None => return Err("ALTREP_CLASS_CATALOGUE is not initialized".into()),
    };

    let mut catalogue = match catalogue_mutex.lock() {
        Ok(catalogue) => catalogue,
        Err(e) => return Err(e.to_string().into()),
    };

    let existing_entry = catalogue.insert(class_name, class_t);

    if existing_entry.is_some() {
        return Err(
            "[WARN] ALTREP class {class_name} is already defined. Something seems wrong.".into(),
        );
    }

    Ok(())
}

pub trait AltInteger {
    /// Class name to identify the ALTREP class.
    const CLASS_NAME: &'static str;

    /// Package name to identify the ALTREP class.
    const PACKAGE_NAME: &'static str;

    /// Copies all the data into a new memory. This is used when the ALTREP
    /// needs to be materialized.
    ///
    /// For example, you can use `copy_from_slice()` for more efficient copying
    /// of the values.
    fn copy_data(&mut self, new: &mut [i32]) {
        for (i, v) in new.iter_mut().enumerate() {
            *v = self.elt(i);
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
    fn elt(&mut self, i: usize) -> i32;
}

#[allow(clippy::mut_from_ref)]
#[inline]
fn extract_self_from_altrep<T>(x: &SEXP) -> &mut T {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_mut() };
    self_.expect("Failed to convert the external pointer to the Rust object")
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn register_altinteger_class<T: 'static + AltInteger>(
    dll_info: *mut crate::ffi::DllInfo,
) -> crate::error::Result<()> {
    let class_name = CString::new(T::CLASS_NAME).unwrap_or_default();
    let package_name = CString::new(T::PACKAGE_NAME).unwrap_or_default();
    let class_t =
        unsafe { R_make_altinteger_class(class_name.as_ptr(), package_name.as_ptr(), dll_info) };

    #[allow(clippy::mut_from_ref)]
    #[inline]
    fn materialize<T: 'static + AltInteger>(x: &SEXP) -> SEXP {
        let data = unsafe { R_altrep_data2(*x) };
        if unsafe { data != R_NilValue } {
            return data;
        }

        let self_: &mut T = extract_self_from_altrep(x);

        let len = self_.length();
        let new = crate::alloc_vector(INTSXP, len).unwrap();

        unsafe { Rf_protect(new) };

        let dst = unsafe { std::slice::from_raw_parts_mut(INTEGER(new), len) };

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

    unsafe extern "C" fn altrep_duplicate<T: 'static + AltInteger>(
        x: SEXP,
        _deep_copy: Rboolean,
    ) -> SEXP {
        let materialized = materialize::<T>(&x);

        // let attrs = unsafe { Rf_protect(Rf_duplicate(ATTRIB(x))) };
        // unsafe { SET_ATTRIB(materialized, attrs) };

        unsafe { Rf_duplicate(materialized) }
    }

    unsafe extern "C" fn altrep_coerce<T: 'static + AltInteger>(
        x: SEXP,
        sexp_type: SEXPTYPE,
    ) -> SEXP {
        let materialized = materialize::<T>(&x);
        unsafe { Rf_coerceVector(materialized, sexp_type) }
    }

    unsafe extern "C" fn altvec_dataptr<T: 'static + AltInteger>(
        x: SEXP,
        _writable: Rboolean,
    ) -> *mut c_void {
        let materialized = materialize::<T>(&x);
        unsafe { INTEGER(materialized) as _ }
    }

    unsafe extern "C" fn altvec_dataptr_or_null<T: 'static + AltInteger>(x: SEXP) -> *const c_void {
        let materialized = materialize::<T>(&x);
        unsafe { INTEGER(materialized) as _ }
    }

    unsafe extern "C" fn altrep_length<T: 'static + AltInteger>(x: SEXP) -> R_xlen_t {
        let self_: &mut T = extract_self_from_altrep(&x);
        self_.length() as _
    }

    unsafe extern "C" fn altrep_inspect<T: 'static + AltInteger>(
        x: SEXP,
        _: i32,
        _: i32,
        _: i32,
        _: Option<unsafe extern "C" fn(SEXP, i32, i32, i32)>,
    ) -> Rboolean {
        let self_: &mut T = extract_self_from_altrep(&x);
        self_.inspect();

        Rboolean_TRUE
    }

    unsafe extern "C" fn altinteger_elt<T: 'static + AltInteger>(
        x: SEXP,
        i: R_xlen_t,
    ) -> std::os::raw::c_int {
        let self_: &mut T = extract_self_from_altrep(&x);
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

    register_altrep_class(T::CLASS_NAME, class_t)?;
    Ok(())
}
