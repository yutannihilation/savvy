use std::ffi::CString;

use savvy_ffi::{
    Rf_eval, Rf_install, Rf_protect, Rf_unprotect, CDR, LANGSXP, SETCAR, SET_TAG, SEXP,
};

use crate::{alloc_vector, protect, unwind_protect, EnvironmentSexp};

use super::Sexp;

/// An external SEXP of a function.
pub struct FunctionSexp(pub SEXP);

/// A result of a function call. Since the result does not yet belong to any
/// environemnt or object, so it needs protection and unprotection. This struct
/// is solely for handling the unprotection in `Drop`.
pub struct FunctionCallResult {
    inner: SEXP,
    token: SEXP,
}

impl FunctionCallResult {
    pub fn inner(&self) -> SEXP {
        self.inner
    }
}

impl Drop for FunctionCallResult {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl From<FunctionCallResult> for Sexp {
    fn from(value: FunctionCallResult) -> Self {
        Self(value.inner())
    }
}

impl From<FunctionCallResult> for crate::error::Result<Sexp> {
    fn from(value: FunctionCallResult) -> Self {
        Ok(<Sexp>::from(value))
    }
}

impl FunctionSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    pub fn call<S, T>(
        &self,
        args: T,
        env: &EnvironmentSexp,
    ) -> crate::error::Result<FunctionCallResult>
    where
        S: AsRef<str>,
        T: Iterator<Item = (S, Sexp)> + ExactSizeIterator,
    {
        unsafe {
            let call = Rf_protect(alloc_vector(LANGSXP, args.len() + 1)?);
            SETCAR(call, self.inner());

            let mut cur = CDR(call);

            for (arg_name, arg_value) in args {
                SETCAR(cur, arg_value.0);

                let arg_name = arg_name.as_ref();
                if !arg_name.is_empty() {
                    let arg_name_cstr = match CString::new(arg_name) {
                        Ok(cstr) => cstr,
                        Err(e) => return Err(crate::error::Error::new(&e.to_string())),
                    };
                    SET_TAG(cur, Rf_install(arg_name_cstr.as_ptr()));
                }

                cur = CDR(cur);
            }

            let res = unwind_protect(|| Rf_eval(call, env.inner()))?;
            let token = protect::insert_to_preserved_list(res);
            Rf_unprotect(1);

            Ok(FunctionCallResult { inner: res, token })
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
