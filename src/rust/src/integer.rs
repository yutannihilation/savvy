use libR_sys::{Rf_xlength, ALTREP, INTEGER, INTEGER_ELT, SEXP};

use crate::sexp::Sxp;

pub struct IntegerSxp(SEXP);

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
}

impl TryFrom<SEXP> for IntegerSxp {
    type Error = anyhow::Error;

    fn try_from(value: SEXP) -> anyhow::Result<Self> {
        if !Sxp(value).is_integer() {
            return Err(crate::error::UnextendrError::UnexpectedType("???".to_string()).into());
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
