use std::{ffi::CStr, option::IntoIter};

use libR_sys::{
    R_NamesSymbol, R_NilValue, Rf_allocVector, Rf_getAttrib, Rf_protect, Rf_setAttrib,
    Rf_translateCharUTF8, Rf_xlength, ALTREP, SEXP, STRSXP, TYPEOF, VECSXP, VECTOR_ELT,
};

use crate::{protect, IntegerSxp, LogicalSxp, NullSxp, OwnedStringSxp, RealSxp, StringSxp};

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

    pub fn get(&self, k: &str) -> Option<ListElement> {
        let index = self.keys().position(|e| e == k);
        Some(self.get_by_index_unchecked(index?))
    }

    pub fn get_by_index(&self, i: usize) -> Option<ListElement> {
        if i >= self.len() {
            return None;
        }

        Some(self.get_by_index_unchecked(i))
    }

    pub fn get_by_index_unchecked(&self, i: usize) -> ListElement {
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

    pub fn keys(&self) -> std::vec::IntoIter<&'static str> {
        let names = unsafe { Rf_getAttrib(self.inner(), R_NamesSymbol) };

        let keys: Vec<&'static str> = if names == unsafe { R_NilValue } {
            std::iter::repeat("").take(self.len()).collect()
        } else {
            StringSxp(names).iter().collect()
        };

        keys.into_iter()
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

impl OwnedListSxp {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, k: &str) -> Option<ListElement> {
        self.inner.get(k)
    }

    pub fn get_by_index(&self, i: usize) -> Option<ListElement> {
        self.inner.get_by_index(i)
    }

    pub fn get_by_index_unchecked(&self, i: usize) -> ListElement {
        self.inner.get_by_index_unchecked(i)
    }

    pub fn values(&self) -> ListSxpValueIter {
        self.inner.values()
    }

    pub fn keys(&self) -> std::vec::IntoIter<&'static str> {
        self.inner.keys()
    }

    pub fn iter(&self) -> ListSxpIter {
        self.inner.iter()
    }

    // TODO: このコードだと、else で names を OwnedStringSxp にするしかない
    // 一度 Rf_setAttrib で渡した後は触りたくないので、new()の時にやるようにする
    pub fn set(&mut self, i: usize, k: Option<&str>, v: &str) {
        if let Some(k) = k {
            let names = unsafe { Rf_getAttrib(self.inner.0, R_NamesSymbol) };
            let names = if names == unsafe { R_NilValue } {
                let new_names = OwnedStringSxp::new(self.len());
                new_names.set_elt(i, k);
                for j in 0..self.len() {
                    if j != i {
                        new_names.set_elt(j, "");
                    }
                }
                unsafe { Rf_setAttrib(self.inner.0, R_NamesSymbol, new_names.inner()) };
            } else {
                // TODO
                names
            };
        }
    }

    pub fn new(len: usize) -> Self {
        let out = unsafe { Rf_allocVector(VECSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);

        Self {
            inner: ListSxp(out),
            token,
        }
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

        if i >= self.len {
            return None;
        }

        Some(self.sexp.get_by_index_unchecked(i))
    }
}

type ListSxpIter<'a> = std::iter::Zip<std::vec::IntoIter<&'static str>, ListSxpValueIter<'a>>;
