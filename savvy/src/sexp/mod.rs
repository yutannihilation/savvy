use std::ffi::{CStr, CString};

use savvy_ffi::{
    Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString, Rf_type2char, EXTPTRSXP, SEXP, TYPEOF,
    VECSXP,
};

use crate::{
    IntegerSexp, ListSexp, LogicalSexp, NullSexp, OwnedIntegerSexp, OwnedLogicalSexp,
    OwnedRealSexp, OwnedStringSexp, RealSexp, StringSexp,
};

pub mod external_pointer;
pub mod integer;
pub mod list;
pub mod logical;
pub mod na;
pub mod null;
pub mod real;
pub mod scalar;
pub mod string;

pub struct Sexp(pub SEXP);

impl Sexp {
    // There are two versions of Rf_isString(), but anyway this should be cheap.
    //
    // macro version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/include/Defn.h#L759
    // function version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/main/memory.c#L4460
    pub fn is_string(&self) -> bool {
        unsafe { Rf_isString(self.0) == 1 }
    }

    pub fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.0) == 1 }
    }

    pub fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.0) == 1 }
    }

    pub fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.0) == 1 }
    }

    // There's no test function for VECSXP. Rf_isList() is for pairlist
    pub fn is_list(&self) -> bool {
        unsafe { TYPEOF(self.0) as u32 == VECSXP }
    }

    pub fn is_external_pointer(&self) -> bool {
        unsafe { TYPEOF(self.0) as u32 == EXTPTRSXP }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_human_readable_type_name(&self) -> &'static str {
        unsafe {
            // TODO: replace this `R_typeToChar()` which will be introduced in R 4.4
            let c = Rf_type2char(TYPEOF(self.0) as _);
            CStr::from_ptr(c).to_str().unwrap()
        }
    }
}

pub enum TypedSexp {
    Integer(IntegerSexp),
    Real(RealSexp),
    String(StringSexp),
    Logical(LogicalSexp),
    List(ListSexp),
    Null(NullSexp),
    Other(SEXP),
}

macro_rules! into_typed_sxp {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for TypedSexp {
            fn from(value: $ty) -> Self {
                TypedSexp::$variant(value)
            }
        }
    };
}

into_typed_sxp!(IntegerSexp, Integer);
into_typed_sxp!(RealSexp, Real);
into_typed_sxp!(StringSexp, String);
into_typed_sxp!(LogicalSexp, Logical);
into_typed_sxp!(ListSexp, List);
into_typed_sxp!(NullSexp, Null);

macro_rules! into_typed_sxp_owned {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for TypedSexp {
            fn from(value: $ty) -> Self {
                TypedSexp::$variant(value.as_read_only())
            }
        }
    };
}

into_typed_sxp_owned!(OwnedIntegerSexp, Integer);
into_typed_sxp_owned!(OwnedRealSexp, Real);
into_typed_sxp_owned!(OwnedStringSexp, String);
into_typed_sxp_owned!(OwnedLogicalSexp, Logical);

impl From<TypedSexp> for SEXP {
    fn from(value: TypedSexp) -> Self {
        match value {
            TypedSexp::Null(_) => unsafe { savvy_ffi::R_NilValue },
            TypedSexp::Integer(sxp) => sxp.inner(),
            TypedSexp::Real(sxp) => sxp.inner(),
            TypedSexp::String(sxp) => sxp.inner(),
            TypedSexp::Logical(sxp) => sxp.inner(),
            TypedSexp::List(sxp) => sxp.inner(),
            TypedSexp::Other(sxp) => sxp,
        }
    }
}

impl Sexp {
    pub fn into_typed(self) -> TypedSexp {
        let ty = unsafe { TYPEOF(self.0) };
        match ty as u32 {
            savvy_ffi::INTSXP => TypedSexp::Integer(IntegerSexp(self.0)),
            savvy_ffi::REALSXP => TypedSexp::Real(RealSexp(self.0)),
            savvy_ffi::STRSXP => TypedSexp::String(StringSexp(self.0)),
            savvy_ffi::LGLSXP => TypedSexp::Logical(LogicalSexp(self.0)),
            savvy_ffi::VECSXP => TypedSexp::List(ListSexp(self.0)),
            savvy_ffi::NILSXP => TypedSexp::Null(NullSexp),
            _ => TypedSexp::Other(self.0),
        }
    }

    pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
        let attr_cstr = match CString::new(attr) {
            Ok(cstr) => cstr,
            Err(e) => return Err(crate::error::Error::new(&e.to_string())),
        };
        let attr_sexp = unsafe {
            crate::unwind_protect(|| {
                savvy_ffi::Rf_getAttrib(self.0, savvy_ffi::Rf_install(attr_cstr.as_ptr()))
            })?
        };

        if attr_sexp == unsafe { savvy_ffi::R_NilValue } {
            Ok(None)
        // Bravely assume the "class" attribute is always a valid STRSXP.
        } else {
            Ok(Some(Sexp(attr_sexp)))
        }
    }

    unsafe fn get_string_attrib_by_symbol(&self, attr: SEXP) -> Option<Vec<&'static str>> {
        let sexp = savvy_ffi::Rf_getAttrib(self.0, attr);

        if sexp == unsafe { savvy_ffi::R_NilValue } {
            None
        // Bravely assume the "class" attribute is always a valid STRSXP.
        } else {
            Some(crate::StringSexp(sexp).iter().collect())
        }
    }

    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        unsafe { self.get_string_attrib_by_symbol(savvy_ffi::R_ClassSymbol) }
    }

    pub fn get_names(&self) -> Option<Vec<&'static str>> {
        unsafe { self.get_string_attrib_by_symbol(savvy_ffi::R_NamesSymbol) }
    }

    pub fn set_attrib(&mut self, attr: &str, value: Sexp) -> crate::error::Result<()> {
        let attr_cstr = match CString::new(attr) {
            Ok(cstr) => cstr,
            Err(e) => return Err(crate::error::Error::new(&e.to_string())),
        };
        unsafe {
            crate::unwind_protect(|| {
                savvy_ffi::Rf_setAttrib(self.0, savvy_ffi::Rf_install(attr_cstr.as_ptr()), value.0)
            })?
        };

        Ok(())
    }

    unsafe fn set_string_attrib_by_symbol(
        &mut self,
        attr: SEXP,
        values: &[&str],
    ) -> crate::error::Result<()> {
        let values_sexp: Sexp = values.try_into()?;
        unsafe { crate::unwind_protect(|| savvy_ffi::Rf_setAttrib(self.0, attr, values_sexp.0))? };

        Ok(())
    }

    pub fn set_class(&mut self, classes: &[&str]) -> crate::error::Result<()> {
        unsafe { self.set_string_attrib_by_symbol(savvy_ffi::R_ClassSymbol, classes) }
    }

    pub fn set_names(&mut self, names: &[&str]) -> crate::error::Result<()> {
        unsafe { self.set_string_attrib_by_symbol(savvy_ffi::R_NamesSymbol, names) }
    }
}

pub(crate) fn get_dim_from_sexp(value: SEXP) -> Option<Vec<usize>> {
    let dim_sexp = unsafe { savvy_ffi::Rf_getAttrib(value, savvy_ffi::R_DimSymbol) };

    if dim_sexp == unsafe { savvy_ffi::R_NilValue } {
        None
    // Bravely assume the "dim" attribute is always a valid INTSXP.
    } else {
        Some(
            crate::IntegerSexp(dim_sexp)
                .as_slice()
                .iter()
                .map(|i| *i as _)
                .collect(),
        )
    }
}

pub(crate) fn set_dim_to_sexp(value: SEXP, dim: &[usize]) -> crate::error::Result<()> {
    let dim_sexp: Sexp = dim
        .iter()
        .map(|v| *v as i32)
        .collect::<Vec<i32>>()
        .try_into()?;
    unsafe { savvy_ffi::Rf_setAttrib(value, savvy_ffi::R_DimSymbol, dim_sexp.0) };
    Ok(())
}

macro_rules! impl_common_sexp_ops {
    ($ty: ty) => {
        impl $ty {
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

            pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
                crate::Sexp(self.inner()).get_attrib(attr)
            }

            pub fn get_names(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_names()
            }

            pub fn get_class(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_class()
            }

            pub fn get_dim(&self) -> Option<Vec<usize>> {
                crate::sexp::get_dim_from_sexp(self.inner())
            }
        }
    };
}

macro_rules! impl_common_sexp_ops_owned {
    ($ty: ty) => {
        impl $ty {
            #[inline]
            pub fn inner(&self) -> SEXP {
                self.inner
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.len
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
                crate::Sexp(self.inner()).get_attrib(attr)
            }

            pub fn get_names(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_names()
            }

            pub fn get_class(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_class()
            }

            pub fn get_dim(&self) -> Option<Vec<usize>> {
                crate::sexp::get_dim_from_sexp(self.inner())
            }

            pub fn set_attrib(&mut self, attr: &str, value: Sexp) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_attrib(attr, value)
            }

            pub fn set_class(&mut self, classes: &[&str]) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_class(classes)
            }

            pub fn set_names(&mut self, names: &[&str]) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_names(names)
            }

            pub fn set_dim(&mut self, dim: &[usize]) -> crate::error::Result<()> {
                crate::sexp::set_dim_to_sexp(self.inner(), dim)
            }
        }
    };
}

pub(crate) use impl_common_sexp_ops;
pub(crate) use impl_common_sexp_ops_owned;
