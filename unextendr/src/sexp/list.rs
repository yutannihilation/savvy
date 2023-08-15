use std::{ffi::CStr, option::IntoIter};

use libR_sys::{
    R_NamesSymbol, R_NilValue, Rf_getAttrib, Rf_translateCharUTF8, Rf_xlength, ALTREP, SEXP,
    TYPEOF, VECTOR_ELT,
};

use crate::{IntegerSxp, LogicalSxp, NullSxp, RealSxp, StringSxp};

use super::{na::NotAvailableValue, string::StringSxpIter};

pub struct ListSxp(pub SEXP);
pub struct OwnedListSxp {
    inner: ListSxp,
    token: SEXP,
}

// TODO: This is a dummy stuct just to make the functions like elt() always
// succeed. Maybe replace this with Sxp?
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

    pub fn get(&self, i: usize) -> Option<ListElement> {
        if i >= self.len() {
            return None;
        }

        Some(self.get_unchecked(i))
    }

    pub fn get_unchecked(&self, i: usize) -> ListElement {
        unsafe {
            let e = VECTOR_ELT(self.0, i as _);
            match TYPEOF(e) as u32 {
                libR_sys::INTSXP => ListElement::Integer(IntegerSxp(e)),
                libR_sys::REALSXP => ListElement::Real(RealSxp(e)),
                libR_sys::STRSXP => ListElement::String(StringSxp(e)),
                libR_sys::LGLSXP => ListElement::Logical(LogicalSxp(e)),
                libR_sys::VECSXP => ListElement::List(ListSxp(e)),
                libR_sys::NILSXP => ListElement::Null(NullSxp),
                _ => ListElement::Unsupported(UnsupportedSxp(e)),
            }
        }
    }

    pub fn values(&self) -> ListSxpValueIter {
        ListSxpValueIter {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    pub fn keys(&self) -> Option<ListSxpKyeIter> {
        let names = unsafe { Rf_getAttrib(self.inner(), R_NamesSymbol) };

        if names == unsafe { R_NilValue } {
            None
        } else {
            Some(StringSxp(names).iter())
        }
    }

    pub fn iter(&self) -> ListSxpIter {
        let keys = self.keys();
        let values = self.values();

        std::iter::zip(keys, values)
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

pub struct ListSxpValueIter<'a> {
    pub sexp: &'a ListSxp,
    i: usize,
    len: usize,
}

impl<'a> Iterator for ListSxpValueIter<'a> {
    type Item = ListElement;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        self.sexp.get(i)
    }
}

type ListSxpKyeIter<'a> = StringSxpIter<'a>;

type ListSxpIter<'a> = std::iter::Zip<IntoIter<StringSxpIter<'a>>, ListSxpValueIter<'a>>;
