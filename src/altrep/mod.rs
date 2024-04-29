mod altinteger;
pub use altinteger::*;
mod altreal;
pub use altreal::*;
mod altlogical;
pub use altlogical::*;

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::OnceCell;
use savvy_ffi::{
    altrep::{R_altrep_class_t, R_altrep_data1, R_new_altrep, MARK_NOT_MUTABLE},
    R_NilValue, SEXP,
};

use crate::{protect::local_protect, IntoExtPtrSexp};

static ALTREP_CLASS_CATALOGUE: OnceCell<Mutex<HashMap<&'static str, R_altrep_class_t>>> =
    OnceCell::new();

pub(crate) fn create_altrep_instance<T: IntoExtPtrSexp>(
    x: T,
    class_name: &'static str,
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
    let class = match catalogue.get(class_name) {
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

// Some helpers

#[allow(clippy::mut_from_ref)]
#[inline]
pub(crate) fn extract_self_from_altrep<T>(x: &SEXP) -> &mut T {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_mut() };
    self_.expect("Failed to convert the external pointer to the Rust object")
}
