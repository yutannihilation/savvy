use savvy_ffi::{
    R_NamesSymbol, R_NilValue, Rf_getAttrib, Rf_setAttrib, Rf_xlength, SET_VECTOR_ELT, SEXP,
    VECSXP, VECTOR_ELT,
};

use crate::{protect, OwnedStringSexp, StringSexp};

use super::{Sexp, TypedSexp};

pub struct ListSexp(pub SEXP);
pub struct OwnedListSexp {
    values: ListSexp,
    names: Option<OwnedStringSexp>,
    token: SEXP,
    len: usize,
}

impl ListSexp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, k: &str) -> Option<TypedSexp> {
        let index = self.names_iter().position(|e| e == k);
        Some(self.get_by_index_unchecked(index?))
    }

    pub fn get_by_index(&self, i: usize) -> Option<TypedSexp> {
        if i >= self.len() {
            return None;
        }

        Some(self.get_by_index_unchecked(i))
    }

    pub fn get_by_index_unchecked(&self, i: usize) -> TypedSexp {
        unsafe {
            let e = VECTOR_ELT(self.0, i as _);
            Sexp(e).into_typed()
        }
    }

    pub fn values_iter(&self) -> ListSexpValueIter {
        ListSexpValueIter {
            sexp: self,
            i: 0,
            len: self.len(),
        }
    }

    pub fn names_iter(&self) -> std::vec::IntoIter<&'static str> {
        let names_sexp = unsafe { Rf_getAttrib(self.inner(), R_NamesSymbol) };

        let names: Vec<&'static str> = if names_sexp == unsafe { R_NilValue } {
            std::iter::repeat("").take(self.len()).collect()
        } else {
            StringSexp(names_sexp).iter().collect()
        };

        names.into_iter()
    }

    pub fn iter(&self) -> ListSexpIter {
        let names = self.names_iter();
        let values = self.values_iter();

        std::iter::zip(names, values)
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedListSexp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get(&self, k: &str) -> Option<TypedSexp> {
        self.values.get(k)
    }

    pub fn get_by_index(&self, i: usize) -> Option<TypedSexp> {
        self.values.get_by_index(i)
    }

    pub fn get_by_index_unchecked(&self, i: usize) -> TypedSexp {
        self.values.get_by_index_unchecked(i)
    }

    pub fn values_iter(&self) -> ListSexpValueIter {
        self.values.values_iter()
    }

    pub fn names_iter(&self) -> std::vec::IntoIter<&'static str> {
        self.values.names_iter()
    }

    pub fn inner(&self) -> SEXP {
        self.values.inner()
    }

    pub fn iter(&self) -> ListSexpIter {
        self.values.iter()
    }

    pub fn set_value<T: Into<TypedSexp>>(&mut self, i: usize, v: T) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }

        let v: TypedSexp = v.into();

        unsafe {
            SET_VECTOR_ELT(self.values.inner(), i as _, v.into());
        }

        Ok(())
    }

    pub fn set_name(&mut self, i: usize, k: &str) -> crate::error::Result<()> {
        // OwnedStringSexp::set_elt() checks the length, so don't check here.

        if let Some(names) = self.names.as_mut() {
            names.set_elt(i, k)?;
        }

        Ok(())
    }

    pub fn set_name_and_value<T: Into<TypedSexp>>(
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
            let names = OwnedStringSexp::new(len)?;
            unsafe { Rf_setAttrib(out, R_NamesSymbol, names.inner()) };
            Some(names)
        } else {
            None
        };

        Ok(Self {
            values: ListSexp(out),
            names,
            token,
            len,
        })
    }
}

impl Drop for OwnedListSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sexp> for ListSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_list() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to list");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

// Conversion into SEXP is infallible as it's just extract the inner one.
impl From<ListSexp> for Sexp {
    fn from(value: ListSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedListSexp> for Sexp {
    fn from(value: OwnedListSexp) -> Self {
        Self(value.inner())
    }
}

pub struct ListSexpValueIter<'a> {
    pub sexp: &'a ListSexp,
    i: usize,
    len: usize,
}

impl<'a> Iterator for ListSexpValueIter<'a> {
    type Item = TypedSexp;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        Some(self.sexp.get_by_index_unchecked(i))
    }
}

type ListSexpIter<'a> = std::iter::Zip<std::vec::IntoIter<&'static str>, ListSexpValueIter<'a>>;
