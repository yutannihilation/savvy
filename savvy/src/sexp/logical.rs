use savvy_ffi::{Rf_xlength, LGLSXP, LOGICAL, SET_LOGICAL_ELT, SEXP};

use super::Sexp;
use crate::protect;

pub struct LogicalSexp(pub SEXP);
pub struct OwnedLogicalSexp {
    inner: SEXP,
    token: SEXP,
    len: usize,
    raw: *mut i32,
}

impl LogicalSexp {
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.0) as _ }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(LOGICAL(self.0), self.len()) }
    }

    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}

impl OwnedLogicalSexp {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_read_only(&self) -> LogicalSexp {
        LogicalSexp(self.inner)
    }

    fn as_slice_raw(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.raw, self.len()) }
    }

    pub fn iter(&self) -> LogicalSexpIter {
        LogicalSexpIter {
            iter_raw: self.as_slice_raw().iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }

    pub fn inner(&self) -> SEXP {
        self.inner
    }

    pub fn set_elt(&mut self, i: usize, v: bool) -> crate::error::Result<()> {
        if i >= self.len {
            return Err(crate::error::Error::new(&format!(
                "index out of bounds: the length is {} but the index is {}",
                self.len, i
            )));
        }

        unsafe {
            SET_LOGICAL_ELT(self.inner, i as _, v as _);
        }

        Ok(())
    }

    pub fn new(len: usize) -> crate::error::Result<Self> {
        let inner = crate::alloc_vector(LGLSXP, len as _)?;
        Self::new_from_raw_sexp(inner, len)
    }

    fn new_from_raw_sexp(inner: SEXP, len: usize) -> crate::error::Result<Self> {
        let token = protect::insert_to_preserved_list(inner);
        let raw = unsafe { LOGICAL(inner) };
        Ok(Self {
            inner,
            token,
            len,
            raw,
        })
    }
}

impl Drop for OwnedLogicalSexp {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl TryFrom<Sexp> for LogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_logical() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Cannot convert {type_name} to logical");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<LogicalSexp> for Sexp {
    fn from(value: LogicalSexp) -> Self {
        Self(value.inner())
    }
}

impl TryFrom<&[bool]> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: &[bool]) -> crate::error::Result<Self> {
        let mut out = Self::new(value.len())?;
        for (i, v) in value.iter().enumerate() {
            out.set_elt(i, *v)?;
        }
        Ok(out)
    }
}

impl TryFrom<Vec<bool>> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: Vec<bool>) -> crate::error::Result<Self> {
        <Self>::try_from(value.as_slice())
    }
}

impl TryFrom<bool> for OwnedLogicalSexp {
    type Error = crate::error::Error;

    fn try_from(value: bool) -> crate::error::Result<Self> {
        let sexp = unsafe { crate::unwind_protect(|| savvy_ffi::Rf_ScalarLogical(value as i32))? };
        Self::new_from_raw_sexp(sexp, 1)
    }
}

macro_rules! impl_try_from_rust_reals {
    ($ty: ty) => {
        impl TryFrom<$ty> for Sexp {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> crate::error::Result<Self> {
                <OwnedLogicalSexp>::try_from(value).map(|x| x.into())
            }
        }
    };
}

impl_try_from_rust_reals!(&[bool]);
impl_try_from_rust_reals!(Vec<bool>);
impl_try_from_rust_reals!(bool);

impl From<OwnedLogicalSexp> for Sexp {
    fn from(value: OwnedLogicalSexp) -> Self {
        Self(value.inner())
    }
}

// I learned implementing the Index trait is wrong; the Index is to provide a
// view of some exisitng object. SEXP can be an ALTREP, which doesn't allocate
// all the values yet.
//
//     impl Index<usize> for LogicalSexp {
//         type Output = i32;
//         fn index(&self, index: usize) -> &Self::Output {
//             &self.elt(index).clone()
//         }
//     }

pub struct LogicalSexpIter<'a> {
    iter_raw: std::slice::Iter<'a, i32>,
}

impl<'a> Iterator for LogicalSexpIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_raw.next().map(|x| *x == 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter_raw.size_hint()
    }
}

impl<'a> ExactSizeIterator for LogicalSexpIter<'a> {
    fn len(&self) -> usize {
        self.iter_raw.len()
    }
}
