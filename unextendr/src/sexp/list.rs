use libR_sys::{Rf_xlength, ALTREP, SEXP, TYPEOF, VECTOR_ELT};

use crate::{IntegerSxp, LogicalSxp, NullSxp, RealSxp, StringSxp};

pub struct ListSxp(SEXP);
pub struct OwnedListSxp {
    inner: ListSxp,
    token: SEXP,
}

// TODO: This is a dummy stuct just to make the functions like elt() always
// succeed.
pub struct UnsupportedSxp(SEXP);

pub enum ListElement {
    Integer(IntegerSxp),
    Real(RealSxp),
    String(StringSxp),
    Logical(LogicalSxp),
    List(ListSxp),
    Null(NullSxp),
    Unsupported(UnsupportedSxp),
}

impl ListSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub(crate) fn elt(&self, i: usize) -> ListElement {
        unsafe {
            let e = VECTOR_ELT(self.0, i as _);
            match TYPEOF(e) as u32 {
                libR_sys::INTSXP => ListElement::Integer(IntegerSxp::from_raw(e)),
                libR_sys::REALSXP => ListElement::Real(RealSxp::from_raw(e)),
                libR_sys::STRSXP => ListElement::String(StringSxp::from_raw(e)),
                libR_sys::LGLSXP => ListElement::Logical(LogicalSxp::from_raw(e)),
                libR_sys::VECSXP => ListElement::List(ListSxp(e)),
                libR_sys::NILSXP => ListElement::Null(NullSxp),
                _ => ListElement::Unsupported(UnsupportedSxp(e)),
            }
        }
    }

    pub fn iter(&self) -> ListSxpIter {
        ListSxpIter {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    fn inner(&self) -> SEXP {
        self.0
    }
}

pub struct ListSxpIter<'a> {
    pub sexp: &'a ListSxp,
    i: usize,
    len: usize,
}

impl<'a> Iterator for ListSxpIter<'a> {
    type Item = ListElement;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        Some(self.sexp.elt(i))
    }
}
