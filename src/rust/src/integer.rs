use libR_sys::{
    Rf_allocVector, Rf_xlength, ALTREP, INTEGER, INTEGER_ELT, INTSXP, SET_INTEGER_ELT, SEXP, STRSXP,
};

use crate::{error::get_human_readable_type_name, protect, sexp::Sxp};

// This is based on the idea of cpp11's `writable`.
//
// `IntegerSxp` is a read-only wrapper for SEXPs provided from outside of Rust;
// since it's the caller's responsibility to PROTECT it, we don't protect it on
// Rust's side.
//
// `OwnedIntegerSxp` is a writable wrapper for SEXPs newly allocated on Rust's
// side. Since it's us who produce it, we protect it and drop it.
pub struct IntegerSxp(SEXP);
pub struct OwnedIntegerSxp {
    inner: IntegerSxp,
    token: SEXP,
}

impl IntegerSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> i32 {
        unsafe { INTEGER_ELT(self.0, i as _) }
    }

    pub fn iter(&self) -> IntegerSxpIter {
        // if the vector is an ALTREP, we cannot directly access the underlying
        // data.
        let raw = unsafe {
            if ALTREP(self.0) == 1 {
                std::ptr::null()
            } else {
                INTEGER(self.0)
            }
        };

        IntegerSxpIter {
            sexp: self,
            raw,
            i: 0,
            len: self.len(),
        }
    }

    fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedIntegerSxp {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn elt(&self, i: usize) -> i32 {
        self.inner.elt(i)
    }

    pub fn iter(&self) -> IntegerSxpIter {
        self.inner.iter()
    }

    pub(crate) fn inner(&self) -> SEXP {
        self.inner.inner()
    }

    pub fn set_elt(&mut self, i: usize, v: i32) {
        unsafe {
            SET_INTEGER_ELT(self.inner(), i as _, v);
        }
    }

    pub fn new(len: usize) -> Self {
        let out = unsafe { Rf_allocVector(INTSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);
        Self {
            inner: IntegerSxp(out),
            token,
        }
    }
}

impl Drop for OwnedIntegerSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<SEXP> for IntegerSxp {
    type Error = anyhow::Error;

    fn try_from(value: SEXP) -> anyhow::Result<Self> {
        if !Sxp(value).is_integer() {
            let type_name = get_human_readable_type_name(value);
            let msg = format!("Cannot convert {type_name} to integer");
            return Err(crate::error::UnextendrError::UnexpectedType(msg).into());
        }
        Ok(Self(value))
    }
}

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for IntegerSxp {
//         type Output = i32;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct IntegerSxpIter<'a> {
    pub sexp: &'a IntegerSxp,
    raw: *const i32,
    i: usize,
    len: usize,
}

impl<'a> Iterator for IntegerSxpIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        if self.raw.is_null() {
            // When ALTREP, access to the value via *_ELT()
            Some(self.sexp.elt(i))
        } else {
            // When non-ALTREP, access to the raw pointer
            unsafe { Some(*(self.raw.add(i))) }
        }
    }
}
