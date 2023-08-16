use libR_sys::{
    R_NamesSymbol, R_NilValue, Rf_allocVector, Rf_getAttrib, Rf_setAttrib, Rf_xlength,
    SET_VECTOR_ELT, SEXP, TYPEOF, VECSXP, VECTOR_ELT,
};

use crate::{protect, IntegerSxp, LogicalSxp, NullSxp, OwnedStringSxp, RealSxp, StringSxp};

pub struct ListSxp(pub SEXP);
pub struct OwnedListSxp {
    values: ListSxp,
    names: Option<OwnedStringSxp>,
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
        self.values.len()
    }

    pub fn get(&self, k: &str) -> Option<ListElement> {
        self.values.get(k)
    }

    pub fn get_by_index(&self, i: usize) -> Option<ListElement> {
        self.values.get_by_index(i)
    }

    pub fn get_by_index_unchecked(&self, i: usize) -> ListElement {
        self.values.get_by_index_unchecked(i)
    }

    pub fn values(&self) -> ListSxpValueIter {
        self.values.values()
    }

    pub fn keys(&self) -> std::vec::IntoIter<&'static str> {
        self.values.keys()
    }

    pub fn iter(&self) -> ListSxpIter {
        self.values.iter()
    }

    pub fn set_value(&mut self, i: usize, v: ListElement) {
        let v = match v {
            ListElement::Null(_) => return,
            ListElement::Integer(e) => e.inner(),
            ListElement::Real(e) => e.inner(),
            ListElement::String(e) => e.inner(),
            ListElement::Logical(e) => e.inner(),
            ListElement::List(e) => e.inner(),
            ListElement::Unsupported(e) => e.0,
        };

        unsafe {
            SET_VECTOR_ELT(self.values.inner(), i as _, v);
        }
    }

    pub fn set_name(&mut self, i: usize, k: &str) {
        if let Some(names) = self.names.as_mut() {
            names.set_elt(i, k);
        }
    }

    pub fn new(len: usize, named: bool) -> Self {
        let out = unsafe { Rf_allocVector(VECSXP, len as _) };
        let token = protect::insert_to_preserved_list(out);

        let names = if named {
            let names = OwnedStringSxp::new(len);
            unsafe { Rf_setAttrib(out, R_NamesSymbol, names.inner()) };
            Some(names)
        } else {
            None
        };

        Self {
            values: ListSxp(out),
            names,
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

impl Drop for OwnedListSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}
