use std::{collections::HashMap, ffi::CString, sync::Mutex};

use once_cell::sync::OnceCell;
use savvy_ffi::{
    altrep::{
        R_altrep_class_t, R_altrep_data1, R_make_altinteger_class, R_new_altrep,
        R_set_altinteger_Elt_method, R_set_altrep_Length_method, MARK_NOT_MUTABLE,
    },
    R_NilValue, R_xlen_t, SEXP,
};

use crate::{protect::local_protect, IntoExtPtrSexp};

static ALTREP_CLASS_CATALOGUE: OnceCell<Mutex<HashMap<&'static str, R_altrep_class_t>>> =
    OnceCell::new();

pub fn create_altrep_instance<T: 'static + AltInteger + IntoExtPtrSexp>(
    x: T,
) -> crate::Result<SEXP> {
    let sexp = x.into_external_pointer().0;
    local_protect(sexp);

    let catalogue_mutex = ALTREP_CLASS_CATALOGUE
        .get()
        .expect("ALTREP_CLASS_CATALOGUE must be initialized before calling this function");
    let catalogue = catalogue_mutex.lock().expect("Failed to get the lock");
    let class = catalogue
        .get(T::CLASS_NAME)
        .expect("Failed to get the ALTREP class");

    let altrep = unsafe { R_new_altrep(*class, sexp, R_NilValue) };
    local_protect(altrep);
    unsafe { MARK_NOT_MUTABLE(altrep) };

    Ok(altrep)
}

fn register_altrep_class(class_name: &'static str, class_t: R_altrep_class_t) {
    // There's no way to let global
    ALTREP_CLASS_CATALOGUE.get_or_init(|| Mutex::new(HashMap::new()));

    let mut catalogue = ALTREP_CLASS_CATALOGUE.get().unwrap().lock().unwrap();

    if catalogue.insert(class_name, class_t).is_some() {
        crate::io::r_eprint(
            "[WARN] ALTREP class {class_name} is already defined. Something seems wrong.\n",
            false,
        );
    }
}

pub trait AltInteger {
    const CLASS_NAME: &'static str;
    fn length(&mut self) -> usize;
    fn elt(&mut self, i: usize) -> i32;
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn register_altinteger_class<T: 'static + AltInteger>(
    dll_info: *mut crate::ffi::DllInfo,
) {
    let class_cstr = CString::new(T::CLASS_NAME).unwrap_or_default();
    let class_t = unsafe {
        R_make_altinteger_class(
            class_cstr.as_ptr(),
            c"savvy-altvec-test-package".as_ptr(),
            dll_info,
        )
    };

    #[allow(clippy::mut_from_ref)]
    #[inline]
    fn extract_self<T>(x: &SEXP) -> &mut T {
        let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
        let res = unsafe { x.as_mut() };
        res.expect("Failed to convert the external pointer to the Rust object")
    }

    unsafe extern "C" fn altrep_length<T: 'static + AltInteger>(x: SEXP) -> R_xlen_t {
        let self_: &mut T = extract_self(&x);
        self_.length() as _
    }

    unsafe extern "C" fn altinteger_elt<T: 'static + AltInteger>(
        arg1: SEXP,
        arg2: R_xlen_t,
    ) -> std::os::raw::c_int {
        let self_: &mut T = extract_self(&arg1);
        self_.elt(arg2 as _) as _
    }

    unsafe {
        R_set_altrep_Length_method(class_t, Some(altrep_length::<T>));
        // R_set_altinteger_No_NA_method(class_t, None);
        // R_set_altinteger_Is_sorted_method(class_t, None);
        // R_set_altinteger_Sum_method(class_t, None);
        // R_set_altinteger_Min_method(class_t, None);
        // R_set_altinteger_Max_method(class_t, None);
        R_set_altinteger_Elt_method(class_t, Some(altinteger_elt::<T>));
    }

    register_altrep_class(T::CLASS_NAME, class_t);
}
