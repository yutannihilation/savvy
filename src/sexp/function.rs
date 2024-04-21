use std::ffi::CString;

use savvy_ffi::{
    R_NilValue, Rf_cons, Rf_eval, Rf_install, Rf_lcons, CDR, SETCAR, SETCDR, SET_TAG, SEXP,
};

use crate::{
    protect::{self, local_protect},
    unwind_protect, EvalResult, ListSexp,
};

use super::Sexp;

/// An external SEXP of a function.
pub struct FunctionSexp(pub SEXP);

/// A pairlist for function arguments
pub struct FunctionArgs {
    head: SEXP,
    tail: SEXP,
    token: SEXP,
    len: usize,
}

impl FunctionArgs {
    pub fn inner(&self) -> SEXP {
        self.head
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        unsafe {
            let head = Rf_cons(R_NilValue, R_NilValue);
            let token = protect::insert_to_preserved_list(head);
            let tail = head;

            Self {
                head,
                tail,
                token,
                len: 0,
            }
        }
    }

    pub fn add<K, V, E>(&mut self, arg_name: K, arg_value: V) -> crate::error::Result<()>
    where
        K: AsRef<str>,
        V: TryInto<Sexp, Error = E>,
        E: Into<crate::error::Error>,
    {
        // Set the arg value
        let v: Sexp = match arg_value.try_into() {
            Ok(sexp) => sexp,
            Err(e) => return Err(e.into()),
        };
        unsafe {
            // As a pairlist is a linked list, a pairlist is not empty, but has
            // one element filled with NULL. If it's the first time, replace the
            // NULL with the value. If not, append the value to the tail.
            if self.len == 0 {
                SETCAR(self.tail, v.0);
            } else {
                SETCDR(self.tail, Rf_cons(v.0, R_NilValue));
                self.tail = CDR(self.tail);
            }
        }

        // Set the arg name
        let arg_name = arg_name.as_ref();
        if !arg_name.is_empty() {
            let arg_name_cstr = match CString::new(arg_name) {
                Ok(cstr) => cstr,
                Err(e) => return Err(crate::error::Error::new(&e.to_string())),
            };
            unsafe {
                SET_TAG(self.tail, Rf_install(arg_name_cstr.as_ptr()));
            }
        }

        self.len += 1;

        Ok(())
    }

    pub fn from_list<L: Into<ListSexp>>(list: L) -> crate::error::Result<Self> {
        let list: ListSexp = list.into();
        let mut args = Self::new();
        for (k, v) in list.iter() {
            args.add(k, v)?;
        }
        Ok(args)
    }
}

impl Drop for FunctionArgs {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl FunctionSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    /// Execute an R function
    pub fn call(&self, args: FunctionArgs) -> crate::error::Result<EvalResult> {
        unsafe {
            let call = if args.is_empty() {
                Rf_lcons(self.inner(), R_NilValue)
            } else {
                Rf_lcons(self.inner(), args.inner())
            };

            local_protect(call);

            // Note: here, probably the environment doesn't matter at all
            // because the first argument is the function, which preserves the
            // releated environments, itself.
            let res = unwind_protect(|| Rf_eval(call, savvy_ffi::R_GlobalEnv))?;
            let token = protect::insert_to_preserved_list(res);

            Ok(EvalResult { inner: res, token })
        }
    }
}

// conversions from/to FunctionSexp ***************

impl TryFrom<Sexp> for FunctionSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        value.assert_function()?;
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
