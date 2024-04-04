use savvy_ffi::{R_NamesSymbol, Rf_setAttrib, SET_VECTOR_ELT, SEXP, VECSXP, VECTOR_ELT};

use crate::{protect, OwnedStringSexp};

use super::{utils::assert_len, utils::str_to_charsxp, Sexp};

/// An external SEXP of a list.
pub struct ListSexp(pub SEXP);

/// A newly-created SEXP of a list.
pub struct OwnedListSexp {
    values: ListSexp,
    names: Option<OwnedStringSexp>,
    token: SEXP,
    len: usize,
}

impl ListSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }

    pub fn len(&self) -> usize {
        unsafe { savvy_ffi::Rf_xlength(self.inner()) as _ }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, k: &str) -> Option<Sexp> {
        let index = self.names_iter().position(|e| e == k);
        Some(unsafe { self.get_by_index_unchecked(index?) })
    }

    pub fn get_by_index(&self, i: usize) -> Option<Sexp> {
        if i >= self.len() {
            return None;
        }

        Some(unsafe { self.get_by_index_unchecked(i) })
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn get_by_index_unchecked(&self, i: usize) -> Sexp {
        unsafe {
            let e = VECTOR_ELT(self.0, i as _);
            Sexp(e)
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
        let names = match crate::Sexp(self.inner()).get_names() {
            Some(names) => names,
            None => std::iter::repeat("").take(self.len()).collect(),
        };

        names.into_iter()
    }

    pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
        crate::Sexp(self.inner()).get_attrib(attr)
    }

    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        crate::Sexp(self.inner()).get_class()
    }

    pub fn iter(&self) -> ListSexpIter {
        let names = self.names_iter();
        let values = self.values_iter();

        std::iter::zip(names, values)
    }
}

impl OwnedListSexp {
    #[inline]
    pub fn inner(&self) -> SEXP {
        self.values.inner()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> ListSexp {
        ListSexp(self.inner())
    }

    pub fn get(&self, k: &str) -> Option<Sexp> {
        self.values.get(k)
    }

    pub fn get_by_index(&self, i: usize) -> Option<Sexp> {
        self.values.get_by_index(i)
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn get_by_index_unchecked(&self, i: usize) -> Sexp {
        unsafe { self.values.get_by_index_unchecked(i) }
    }

    pub fn values_iter(&self) -> ListSexpValueIter {
        self.values.values_iter()
    }

    pub fn names_iter(&self) -> std::vec::IntoIter<&'static str> {
        self.values.names_iter()
    }

    pub fn iter(&self) -> ListSexpIter {
        self.values.iter()
    }

    pub fn set_value<T: Into<Sexp>>(&mut self, i: usize, v: T) -> crate::error::Result<()> {
        assert_len(self.len, i)?;

        unsafe { self.set_value_unchecked(i, v.into().0) };

        Ok(())
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    #[inline]
    pub unsafe fn set_value_unchecked(&mut self, i: usize, v: SEXP) {
        unsafe { SET_VECTOR_ELT(self.values.inner(), i as _, v) };
    }

    pub fn set_name(&mut self, i: usize, k: &str) -> crate::error::Result<()> {
        assert_len(self.len, i)?;

        unsafe { self.set_name_unchecked(i, str_to_charsxp(k)?) };

        Ok(())
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    #[inline]
    pub unsafe fn set_name_unchecked(&mut self, i: usize, k: SEXP) {
        if let Some(names) = self.names.as_mut() {
            unsafe { names.set_elt_unchecked(i, k) };
        }
    }

    pub fn set_name_and_value<T: Into<Sexp>>(
        &mut self,
        i: usize,
        k: &str,
        v: T,
    ) -> crate::error::Result<()> {
        self.set_name(i, k)?;
        // cast unchecked function since set_name already does bounds check.
        unsafe { self.set_value_unchecked(i, v.into().0) };
        Ok(())
    }

    pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
        crate::Sexp(self.inner()).get_attrib(attr)
    }

    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        crate::Sexp(self.inner()).get_class()
    }

    pub fn set_attrib(&mut self, attr: &str, value: Sexp) -> crate::error::Result<()> {
        crate::Sexp(self.inner()).set_attrib(attr, value)
    }

    pub fn set_class<T: AsRef<str>>(&mut self, classes: &[T]) -> crate::error::Result<()> {
        crate::Sexp(self.inner()).set_class(classes)
    }

    /// Constructs a new list.
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
        value.assert_list()?;
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

impl From<OwnedListSexp> for ListSexp {
    fn from(value: OwnedListSexp) -> Self {
        value.as_read_only()
    }
}

// Iterator for ListSexp ***************

pub struct ListSexpValueIter<'a> {
    pub sexp: &'a ListSexp,
    i: usize,
    len: usize,
}

impl<'a> Iterator for ListSexpValueIter<'a> {
    type Item = Sexp;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i >= self.len {
            return None;
        }

        Some(unsafe { self.sexp.get_by_index_unchecked(i) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for ListSexpValueIter<'a> {}

type ListSexpIter<'a> = std::iter::Zip<std::vec::IntoIter<&'static str>, ListSexpValueIter<'a>>;
