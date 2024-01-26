use savvy_ffi::{Rf_eval, Rf_protect, Rf_unprotect, LANGSXP, SETCAR, SEXP};

use crate::{alloc_vector, EnvironmentSexp};

use super::Sexp;

/// An external SEXP of a function.
pub struct FunctionSexp(pub SEXP);

impl FunctionSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    pub fn call(&self, env: &EnvironmentSexp) -> crate::error::Result<Sexp> {
        unsafe {
            let call = Rf_protect(alloc_vector(LANGSXP, 1)?);
            SETCAR(call, self.inner());
            let res = Rf_eval(call, env.inner());
            Rf_unprotect(1);

            Ok(Sexp(res))
        }
    }
}

// conversions from/to FunctionSexp ***************

impl TryFrom<Sexp> for FunctionSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_function() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected a function, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<FunctionSexp> for Sexp {
    fn from(value: FunctionSexp) -> Self {
        Self(value.inner())
    }
}

impl From<FunctionSexp> for crate::error::Result<Sexp> {
    fn from(value: FunctionSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}
