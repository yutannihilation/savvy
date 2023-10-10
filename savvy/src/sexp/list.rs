use libR_sys::{
    R_NamesSymbol, R_NilValue, Rf_getAttrib, Rf_protect, Rf_setAttrib, Rf_unprotect, Rf_xlength,
    SET_VECTOR_ELT, SEXP, TYPEOF, VECSXP, VECTOR_ELT,
};

use crate::{
    protect, IntegerSxp, LogicalSxp, NullSxp, OwnedIntegerSxp, OwnedLogicalSxp, OwnedRealSxp,
    OwnedStringSxp, RealSxp, StringSxp,
};

use super::Sxp;

pub struct ListSxp(pub SEXP);
pub struct OwnedListSxp {
    values: ListSxp,
    names: Option<OwnedStringSxp>,
    token: SEXP,
    len: usize,
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

macro_rules! into_list_elem {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for ListElement {
            fn from(value: $ty) -> Self {
                ListElement::$variant(value)
            }
        }
    };
}

into_list_elem!(IntegerSxp, Integer);
into_list_elem!(RealSxp, Real);
into_list_elem!(StringSxp, String);
into_list_elem!(LogicalSxp, Logical);
into_list_elem!(ListSxp, List);
into_list_elem!(NullSxp, Null);

macro_rules! into_list_elem_owned {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for ListElement {
            fn from(value: $ty) -> Self {
                ListElement::$variant(value.as_read_only())
            }
        }
    };
}

into_list_elem_owned!(OwnedIntegerSxp, Integer);
into_list_elem_owned!(OwnedRealSxp, Real);
into_list_elem_owned!(OwnedStringSxp, String);
into_list_elem_owned!(OwnedLogicalSxp, Logical);

impl From<ListElement> for SEXP {
    fn from(value: ListElement) -> Self {
        match value {
            ListElement::Null(e) => e.into(),
            ListElement::Integer(e) => e.inner(),
            ListElement::Real(e) => e.inner(),
            ListElement::String(e) => e.inner(),
            ListElement::Logical(e) => e.inner(),
            ListElement::List(e) => e.inner(),
            ListElement::Unsupported(e) => e.0,
        }
    }
}

impl ListSxp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, k: &str) -> Option<ListElement> {
        let index = self.names_iter().position(|e| e == k);
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

    pub fn values_iter(&self) -> ListSxpValueIter {
        ListSxpValueIter {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    // TODO: can this return &'a str?
    pub fn names_iter(&self) -> std::vec::IntoIter<String> {
        let names_sexp = unsafe { Rf_protect(Rf_getAttrib(self.inner(), R_NamesSymbol)) };

        let names: Vec<String> = if names_sexp == unsafe { R_NilValue } {
            std::iter::repeat("".to_string()).take(self.len()).collect()
        } else {
            StringSxp(names_sexp)
                .iter()
                .map(|x| x.to_string())
                .collect()
        };

        unsafe {
            Rf_unprotect(1);
        }

        names.into_iter()
    }

    pub fn iter(&self) -> ListSxpIter {
        let names = self.names_iter();
        let values = self.values_iter();

        std::iter::zip(names, values)
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedListSxp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
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

    pub fn values_iter(&self) -> ListSxpValueIter {
        self.values.values_iter()
    }

    pub fn names_iter(&self) -> std::vec::IntoIter<String> {
        self.values.names_iter()
    }

    pub fn inner(&self) -> SEXP {
        self.values.inner()
    }

    pub fn iter(&self) -> ListSxpIter {
        self.values.iter()
    }

    pub fn set_value<T: Into<ListElement>>(&mut self, i: usize, v: T) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }

        let v: ListElement = v.into();

        unsafe {
            SET_VECTOR_ELT(self.values.inner(), i as _, v.into());
        }

        Ok(())
    }

    pub fn set_name(&mut self, i: usize, k: &str) -> crate::error::Result<()> {
        // OwnedStringSxp::set_elt() checks the length, so don't check here.

        if let Some(names) = self.names.as_mut() {
            names.set_elt(i, k)?;
        }

        Ok(())
    }

    pub fn set_name_and_value<T: Into<ListElement>>(
        &mut self,
        i: usize,
        k: &str,
        v: T,
    ) -> crate::error::Result<()> {
        self.set_name(i, k)?;
        self.set_value(i, v)?;
        Ok(())
    }

    pub fn new(len: usize, named: bool) -> crate::error::Result<Self> {
        let out = crate::alloc_vector(VECSXP, len as _)?;
        let token = protect::insert_to_preserved_list(out);

        let names = if named {
            let names = OwnedStringSxp::new(len)?;
            unsafe { Rf_setAttrib(out, R_NamesSymbol, names.inner()) };
            Some(names)
        } else {
            None
        };

        Ok(Self {
            values: ListSxp(out),
            names,
            token,
            len,
        })
    }
}

impl Drop for OwnedListSxp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sxp> for ListSxp {
    type Error = crate::error::Error;

    fn try_from(value: Sxp) -> crate::error::Result<Self> {
        if !value.is_list() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to list");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<ListSxp> for SEXP {
    fn from(value: ListSxp) -> Self {
        value.inner()
    }
}

impl From<OwnedListSxp> for SEXP {
    fn from(value: OwnedListSxp) -> Self {
        value.inner()
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

type ListSxpIter<'a> = std::iter::Zip<std::vec::IntoIter<String>, ListSxpValueIter<'a>>;
