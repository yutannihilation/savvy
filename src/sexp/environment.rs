use std::ffi::CString;

use savvy_ffi::{R_GlobalEnv, R_NilValue, R_UnboundValue, Rboolean_TRUE, SEXP};

use crate::{savvy_err, Sexp};

use super::utils::str_to_symsxp;

/// An environment.
pub struct EnvironmentSexp(pub SEXP);

impl EnvironmentSexp {
    /// Returns the raw SEXP.
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    /// Returns the SEXP bound to a variable of the specified name in the
    /// specified environment.
    ///
    /// The absense of an object with the specified name is represented as
    /// `None`. `Some(NilSexp)` means there's a variable whose value is `NULL`.
    ///
    /// # Protection
    ///
    /// The result `Sexp` is unprotected. In most of the cases, you don't need
    /// to worry about this because existing in an environment means it won't be
    /// GC-ed as long as the environment exists (it's possible the correspondig
    /// variable gets explicitly removed, but it should be rare). However, if
    /// the environment is a temporary one (e.g. an exectuion environment of a
    /// function call), it's your responsibility to protect the object. In other
    /// words, you should never use this if you don't understand how R's
    /// protection mechanism works.
    pub fn get<T: AsRef<str>>(&self, name: T) -> crate::error::Result<Option<crate::Sexp>> {
        let sym = str_to_symsxp(name)?.ok_or(savvy_err!("name must not be empty"))?;

        // Note: since this SEXP already belongs to an environment, this doesn't
        // need protection.
        let sexp = unsafe {
            crate::unwind_protect(|| {
                if savvy_ffi::R_existsVarInFrame(self.0, sym) == Rboolean_TRUE {
                    // TODO: replace this with R_getVar() when savvy drop supports on R <4.5
                    savvy_ffi::Rf_eval(sym, self.0)
                } else {
                    R_UnboundValue
                }
            })?
        };

        if sexp == unsafe { R_UnboundValue } {
            Ok(None)
        } else {
            Ok(Some(Sexp(sexp)))
        }
    }

    /// Returns `true` the specified environment contains the specified
    /// variable.
    pub fn contains<T: AsRef<str>>(&self, name: T) -> crate::error::Result<bool> {
        let sym = str_to_symsxp(name)?.ok_or(savvy_err!("name must not be empty"))?;

        let res = unsafe {
            crate::unwind_protect(|| {
                if savvy_ffi::R_existsVarInFrame(self.0, sym) == Rboolean_TRUE {
                    // Note: Since `unwind_protect()` can only return an SEXP,
                    // this needs to be some sentinel value. Any SEXP can be
                    // used here as long as it can be used as a signal of a
                    // success.
                    R_NilValue
                } else {
                    R_UnboundValue
                }
            })? == R_NilValue
        };

        Ok(res)
    }

    /// Bind the SEXP to the specified environment as the specified name.
    pub fn set<T: AsRef<str>>(&self, name: T, value: Sexp) -> crate::error::Result<()> {
        let name_cstr = CString::new(name.as_ref())?;

        unsafe {
            crate::unwind_protect(|| {
                savvy_ffi::Rf_defineVar(savvy_ffi::Rf_install(name_cstr.as_ptr()), value.0, self.0);
                R_NilValue
            })?
        };

        Ok(())
    }

    /// Return the global env.
    pub fn global_env() -> Self {
        Self(unsafe { R_GlobalEnv })
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
