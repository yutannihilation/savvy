use std::ffi::CString;

use savvy_ffi::{R_NilValue, R_UnboundValue, Rboolean_FALSE, Rboolean_TRUE, SEXP};

use crate::Sexp;

use super::utils::str_to_symsxp;

/// An environment.
pub struct EnvironmentSexp(pub SEXP);

impl EnvironmentSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    pub fn get<T: AsRef<str>>(&self, name: T) -> crate::error::Result<Option<crate::Sexp>> {
        let sym = str_to_symsxp(name)?.ok_or("name must not be empty")?;

        // Note: since this SEXP already belongs to an environment, this doesn't
        // need protection.
        let sexp = unsafe {
            crate::unwind_protect(|| savvy_ffi::Rf_findVarInFrame3(self.0, sym, Rboolean_TRUE))?
        };

        if sexp == unsafe { R_UnboundValue } {
            Ok(None)
        } else {
            Ok(Some(Sexp(sexp)))
        }
    }

    pub fn contains<T: AsRef<str>>(&self, name: T) -> crate::error::Result<bool> {
        let sym = str_to_symsxp(name)?.ok_or("name must not be empty")?;

        let res = unsafe {
            crate::unwind_protect(|| savvy_ffi::Rf_findVarInFrame3(self.0, sym, Rboolean_FALSE))?
                != R_UnboundValue
        };

        Ok(res)
    }

    pub fn set<T: AsRef<str>>(&self, name: T, value: Sexp) -> crate::error::Result<()> {
        let name_cstr = match CString::new(name.as_ref()) {
            Ok(cstr) => cstr,
            Err(e) => return Err(crate::error::Error::new(&e.to_string())),
        };

        unsafe {
            crate::unwind_protect(|| {
                savvy_ffi::Rf_defineVar(savvy_ffi::Rf_install(name_cstr.as_ptr()), value.0, self.0);
                R_NilValue
            })?
        };

        Ok(())
    }
}

// conversions from/to EnvironmentSexp ***************

impl TryFrom<Sexp> for EnvironmentSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_environment()?;
        Ok(Self(value.0))
    }
}

impl From<EnvironmentSexp> for Sexp {
    fn from(value: EnvironmentSexp) -> Self {
        Self(value.inner())
    }
}

impl From<EnvironmentSexp> for crate::error::Result<Sexp> {
    fn from(value: EnvironmentSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}
