use savvy_ffi::{R_NamesSymbol, Rf_setAttrib, SET_VECTOR_ELT, SEXP, VECSXP, VECTOR_ELT};

use crate::{protect, OwnedStringSexp};

use super::{impl_common_sexp_ops, Sexp, TypedSexp};

pub struct ListSexp(pub SEXP);
pub struct OwnedListSexp {
    values: ListSexp,
    names: Option<OwnedStringSexp>,
    token: SEXP,
    len: usize,
}

// implement inner(), len(), empty(), and name()
impl_common_sexp_ops!(ListSexp);

impl ListSexp {
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

    fn names_iter(&self) -> std::vec::IntoIter<&'static str> {
        self.get_names().into_iter()
    }

    pub fn iter(&self) -> ListSexpIter {
        let names = self.names_iter();
        let values = self.values_iter();

        std::iter::zip(names, values)
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

        // Note: `R_allocVector()` initializes lists, so we don't need to do it
        // by ourselves. R-exts (5.9.2 Allocating storage) says:
        //
        // >  One distinction is that whereas the R functions always initialize
        // >  the elements of the vector, allocVector only does so for lists,
        // >  expressions and character vectors (the cases where the elements
        // >  are themselves R objects).

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

// conversions from/to ListSexp ***************

impl TryFrom<Sexp> for ListSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_list() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected a list, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<ListSexp> for Sexp {
    fn from(value: ListSexp) -> Self {
        Self(value.inner())
    }
}

impl From<ListSexp> for crate::error::Result<Sexp> {
    fn from(value: ListSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// conversions from/to OwnedListSexp ***************

impl From<OwnedListSexp> for Sexp {
    fn from(value: OwnedListSexp) -> Self {
        Self(value.inner())
    }
}

impl From<OwnedListSexp> for crate::error::Result<Sexp> {
    fn from(value: OwnedListSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}

// Iterator for ListSexp ***************

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
