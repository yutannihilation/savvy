use std::ffi::CStr;

use libR_sys::{
    R_NamesSymbol, R_NilValue, Rf_getAttrib, Rf_translateCharUTF8, Rf_xlength, ALTREP, SEXP,
    TYPEOF, VECTOR_ELT,
};

use crate::{IntegerSxp, LogicalSxp, NullSxp, RealSxp, StringSxp};

use super::na::NotAvailableValue;

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

    pub(crate) fn elt(&self, i: usize) -> ListElement {
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

    pub fn iter(&self) -> ListSxpIter {
        let names = unsafe { Rf_getAttrib(self.inner(), R_NamesSymbol) };
        let keys = if names == unsafe { R_NilValue } {
            None
        } else {
            Some(StringSxp(names))
        };

        ListSxpIter {
            values: self,
            keys,
            i: 0,
            len: self.len(),
        }
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

pub struct ListSxpIter<'a> {
    values: &'a ListSxp,
    keys: Option<StringSxp>,
    i: usize,
    len: usize,
}

impl<'a> Iterator for ListSxpIter<'a> {
    type Item = (&'a str, ListElement);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        let key = if let Some(StringSxp(k)) = self.keys {
            if k == unsafe { libR_sys::R_NaString } {
                <&str>::na()
            } else {
                unsafe { CStr::from_ptr(Rf_translateCharUTF8(k)).to_str().unwrap() }
            }
        } else {
            <&str>::na()
        };
        let value = self.values.elt(i);

        Some((key, value))
    }
}
