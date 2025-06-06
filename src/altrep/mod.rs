mod altinteger;
mod altlist;
mod altlogical;
mod altraw;
mod altreal;
mod altstring;

pub use altinteger::*;
pub use altlist::*;
pub use altlogical::*;
pub use altraw::*;
pub use altreal::*;
pub use altstring::*;

use std::{collections::HashMap, sync::Mutex};

use savvy_ffi::{
    altrep::{
        R_altrep_class_t, R_altrep_data1, R_altrep_inherits, R_new_altrep, ALTREP, ALTREP_CLASS,
        MARK_NOT_MUTABLE,
    },
    R_NilValue, Rboolean_TRUE, ATTRIB, CADR, CAR, PRINTNAME, SEXP,
};
use std::sync::OnceLock;

use crate::{protect::local_protect, savvy_err, sexp::utils::charsxp_to_str, IntoExtPtrSexp};

/// This stores the ALTREP class objects
static ALTREP_CLASS_CATALOGUE: OnceLock<Mutex<HashMap<&'static str, R_altrep_class_t>>> =
    OnceLock::new();

pub(crate) fn create_altrep_instance<T: IntoExtPtrSexp>(
    x: T,
    class_name: &'static str,
) -> crate::Result<SEXP> {
    let sexp = x.into_external_pointer().0;
    let _sexp_guard = local_protect(sexp);

    let catalogue_mutex = ALTREP_CLASS_CATALOGUE.get().ok_or(crate::Error::new(
        "ALTREP_CLASS_CATALOGUE is not initialized",
    ))?;
    let catalogue = catalogue_mutex.lock()?;
    let class = catalogue
        .get(class_name)
        .ok_or(crate::Error::new("Failed to get the ALTREP class"))?;

    let altrep = unsafe { R_new_altrep(*class, sexp, R_NilValue) };
    let _altrep_guard = local_protect(altrep);

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
    let mut catalogue = catalogue_mutex.lock()?;

    let existing_entry = catalogue.insert(class_name, class_t);

    if existing_entry.is_some() {
        return Err(savvy_err!(
            "[WARN] ALTREP class {class_name} is already defined. Something seems wrong."
        ));
    }

    Ok(())
}

/// Returns the class name of an ALTREP object.
///
/// # Safety
/// This relies on undocumented implementation details of ALTREP, so something
/// unexpected might happen.
pub unsafe fn get_altrep_class_name(x: SEXP) -> crate::error::Result<&'static str> {
    if unsafe { ALTREP(x) } != 1 {
        return Err(savvy_err!("Not an ALTREP"));
    }

    let class_name_symbol = unsafe { CAR(ATTRIB(ALTREP_CLASS(x))) };
    Ok(unsafe { charsxp_to_str(PRINTNAME(class_name_symbol)) })
}

/// Returns the package name of an ALTREP object.
///
/// # Safety
/// This relies on undocumented implementation details of ALTREP, so something
/// unexpected might happen.
pub unsafe fn get_altrep_package_name(x: SEXP) -> crate::error::Result<&'static str> {
    if unsafe { ALTREP(x) } != 1 {
        return Err(savvy_err!("Not an ALTREP"));
    }

    let class_name_symbol = unsafe { CADR(ATTRIB(ALTREP_CLASS(x))) };
    Ok(unsafe { charsxp_to_str(PRINTNAME(class_name_symbol)) })
}

/// Returns the `data1` of an ALTREP object as `&mut`.
///
/// # Safety
/// Using this on a different type of pointer causes undefined behavior. It's
/// user's responsibility to ensure the underlying pointer is `T`.
pub unsafe fn get_altrep_body_mut_unchecked<T>(x: &mut SEXP) -> crate::error::Result<&mut T> {
    if unsafe { ALTREP(*x) } != 1 {
        return Err(savvy_err!("Not an ALTREP"));
    }

    let ptr = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x))? as *mut T };
    let ref_mut = unsafe { ptr.as_mut() };
    ref_mut.ok_or(savvy_err!(
        "Failed to convert the external pointer to the Rust object"
    ))
}

/// Returns the `data1` of an ALTREP object as `&`.
///
/// # Safety
/// Using this on a different type of pointer causes undefined behavior. It's
/// user's responsibility to ensure the underlying pointer is `T`.
pub unsafe fn get_altrep_body_ref_unchecked<T>(x: &SEXP) -> crate::error::Result<&T> {
    if unsafe { ALTREP(*x) } != 1 {
        return Err(savvy_err!("Not an ALTREP"));
    }

    let ptr = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x))? as *const T };
    let ref_mut = unsafe { ptr.as_ref() };
    ref_mut.ok_or(savvy_err!(
        "Failed to convert the external pointer to the Rust object"
    ))
}

// Some helpers

/// Extracts &T
#[inline]
pub(crate) fn extract_ref_from_altrep<T>(x: &SEXP) -> crate::Result<&T> {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_ref() };
    self_.ok_or(savvy_err!(
        "Failed to convert the external pointer to the Rust object"
    ))
}

/// Extracts &mut T
#[allow(clippy::mut_from_ref)]
#[inline]
pub(crate) fn extract_mut_from_altrep<T>(x: &mut SEXP) -> crate::Result<&mut T> {
    let x = unsafe { crate::get_external_pointer_addr(R_altrep_data1(*x)).unwrap() as *mut T };
    let self_ = unsafe { x.as_mut() };
    self_.ok_or(savvy_err!(
        "Failed to convert the external pointer to the Rust object"
    ))
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
    let catalogue = catalogue_mutex.lock()?;
    let class = catalogue
        .get(class_name)
        .ok_or(savvy_err!("Failed to get the ALTREP class"))?;

    if unsafe { R_altrep_inherits(x, *class) == Rboolean_TRUE } {
        Ok(())
    } else {
        Err(savvy_err!(
            "Not an object of the specified ALTREP class ({})",
            class_name
        ))
    }
}
