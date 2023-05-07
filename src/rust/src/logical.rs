use libR_sys::{Rf_xlength, ALTREP, LOGICAL, LOGICAL_ELT, SEXP};

use crate::{error::get_human_readable_type_name, sexp::Sxp};

pub struct LogicalSxp(SEXP);

impl LogicalSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> bool {
        unsafe { LOGICAL_ELT(self.0, i as _) == 1 }
    }

    pub fn iter(&self) -> LogicalSxpIter {
        // if the vector is an ALTREP, we cannot directly access the underlying
        // data.
        let raw = unsafe {
            if ALTREP(self.0) == 1 {
                std::ptr::null()
            } else {
                LOGICAL(self.0)
            }
        };

        LogicalSxpIter {
            sexp: self,
            raw,
            i: 0,
            len: self.len(),
        }
    }
}

impl TryFrom<SEXP> for LogicalSxp {
    type Error = anyhow::Error;

    fn try_from(value: SEXP) -> anyhow::Result<Self> {
        if !Sxp(value).is_logical() {
            let type_name = get_human_readable_type_name(value);
            let msg = format!("Cannot convert {type_name} to logical");
            return Err(crate::error::UnextendrError::UnexpectedType(msg).into());
        }
        Ok(Self(value))
    }
}

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for LogicalSxp {
//         type Output = i32;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct LogicalSxpIter<'a> {
    pub sexp: &'a LogicalSxp,
    raw: *const i32,
    i: usize,
    len: usize,
}

impl<'a> Iterator for LogicalSxpIter<'a> {
    type Item = bool;

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
            unsafe { Some(*(self.raw.add(i)) == 1) }
        }
    }
}
