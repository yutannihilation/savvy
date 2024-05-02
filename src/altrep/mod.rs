mod altinteger;
mod altlogical;
mod altreal;
mod altstring;

pub use altinteger::*;
pub use altlogical::*;
pub use altreal::*;
pub use altstring::*;

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::OnceCell;
use savvy_ffi::{
    altrep::{R_altrep_class_t, R_altrep_data1, R_altrep_inherits, R_new_altrep, MARK_NOT_MUTABLE},
    R_NilValue, Rboolean_TRUE, SEXP,
};

use crate::{protect::local_protect, IntoExtPtrSexp};

/// This stores the ALTREP class objects
static ALTREP_CLASS_CATALOGUE: OnceCell<Mutex<HashMap<&'static str, R_altrep_class_t>>> =
    OnceCell::new();

pub(crate) fn create_altrep_instance<T: IntoExtPtrSexp>(
    x: T,
    class_name: &'static str,
) -> crate::Result<SEXP> {
    let sexp = x.into_external_pointer().0;
    local_protect(sexp);

    let catalogue_mutex = ALTREP_CLASS_CATALOGUE.get().ok_or(crate::Error::new(
        "ALTREP_CLASS_CATALOGUE is not initialized",
    ))?;
    let catalogue = catalogue_mutex
        .lock()
        .map_err(|e| crate::Error::new(&e.to_string()))?;
    let class = catalogue
        .get(class_name)
        .ok_or(crate::Error::new("Failed to get the ALTREP class"))?;

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

    let catalogue_mutex = ALTREP_CLASS_CATALOGUE.get().ok_or(crate::Error::new(
        "ALTREP_CLASS_CATALOGUE is not initialized",
    ))?;
    let mut catalogue = catalogue_mutex
        .lock()
        .map_err(|e| crate::Error::new(&e.to_string()))?;

    let existing_entry = catalogue.insert(class_name, class_t);

    if existing_entry.is_some() {
        return Err(
            "[WARN] ALTREP class {class_name} is already defined. Something seems wrong.".into(),
        );
    }

    Ok(())
}

// Some helpers

/// Extracts &T
#[inline]
pub(crate) fn extract_ref_from_altrep<T>(x: &SEXP) -> crate::Result<&T> {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_ref() };
    self_.ok_or("Failed to convert the external pointer to the Rust object".into())
}

/// Extracts &mut T
#[allow(clippy::mut_from_ref)]
#[inline]
pub(crate) fn extract_mut_from_altrep<T>(x: &mut SEXP) -> crate::Result<&mut T> {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_mut() };
    self_.ok_or("Failed to convert the external pointer to the Rust object".into())
}

/// Extract T
#[inline]
pub(crate) fn extract_from_altrep<T>(x: SEXP) -> crate::Result<T> {
    unsafe { crate::take_external_pointer_value::<T>(R_altrep_data1(x)) }
}

/// Checks if the input is the ALTREP of the supposed ALTREP class assigned to
/// the class name.
#[inline]
pub(crate) fn assert_altrep_class(x: SEXP, class_name: &'static str) -> crate::error::Result<()> {
    let catalogue_mutex = ALTREP_CLASS_CATALOGUE.get().ok_or(crate::Error::new(
        "ALTREP_CLASS_CATALOGUE is not initialized",
    ))?;
    let catalogue = catalogue_mutex
        .lock()
        .map_err(|e| crate::Error::new(&e.to_string()))?;
    let class = catalogue
        .get(class_name)
        .ok_or(crate::Error::new("Failed to get the ALTREP class"))?;

    if unsafe { R_altrep_inherits(x, *class) == Rboolean_TRUE } {
        Ok(())
    } else {
        Err(format!(
            "Not an object of the specified ALTREP class ({})",
            class_name
        )
        .into())
    }
}
