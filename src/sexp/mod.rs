use std::ffi::{CStr, CString};

use savvy_ffi::{
    R_NilValue, Rf_getAttrib, Rf_isFunction, Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString,
    Rf_type2char, Rf_xlength, EXTPTRSXP, INTEGER, SEXP, SEXPTYPE, TYPEOF, VECSXP,
};

use crate::{
    ExternalPointerSexp, IntegerSexp, ListSexp, LogicalSexp, NullSexp, OwnedIntegerSexp,
    OwnedLogicalSexp, OwnedRealSexp, OwnedStringSexp, RealSexp, StringSexp,
};

#[cfg(feature = "complex")]
use crate::{ComplexSexp, OwnedComplexSexp};

pub mod external_pointer;
pub mod function;
pub mod integer;
pub mod list;
pub mod logical;
pub mod na;
pub mod null;
pub mod real;
pub mod scalar;
pub mod string;
pub mod utils;

#[cfg(feature = "complex")]
pub mod complex;

/// An `SEXP`.
pub struct Sexp(pub SEXP);

impl Sexp {
    /// Returns `true` if the SEXP is NULL.
    pub fn is_null(&self) -> bool {
        unsafe { self.0 == R_NilValue }
    }

    /// Returns `true` if the SEXP is an integer vector.
    pub fn is_integer(&self) -> bool {
        unsafe { Rf_isInteger(self.0) == 1 }
    }

    /// Returns `true` if the SEXP is a real vector.
    pub fn is_real(&self) -> bool {
        unsafe { Rf_isReal(self.0) == 1 }
    }

    #[cfg(feature = "complex")]
    /// Returns `true` if the SEXP is a complex.
    pub fn is_complex(&self) -> bool {
        unsafe { savvy_ffi::Rf_isComplex(self.0) == 1 }
    }

    /// Returns `true` if the SEXP is a logical vector.
    pub fn is_logical(&self) -> bool {
        unsafe { Rf_isLogical(self.0) == 1 }
    }

    /// Returns `true` if the SEXP is a character vector.
    pub fn is_string(&self) -> bool {
        // There are two versions of `Rf_isString()``, but anyway this should be cheap.
        //
        // macro version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/include/Defn.h#L759
        // function version: https://github.com/wch/r-source/blob/9065779ee510b7bd8ca93d08f4dd4b6e2bd31923/src/main/memory.c#L4460
        unsafe { Rf_isString(self.0) == 1 }
    }

    /// Returns `true` if the SEXP is a list.
    pub fn is_list(&self) -> bool {
        // There's no test function for VECSXP. Rf_isList() is for pairlist
        unsafe { TYPEOF(self.0) as u32 == VECSXP }
    }

    /// Returns `true` if the SEXP is an external pointer.
    pub fn is_external_pointer(&self) -> bool {
        unsafe { TYPEOF(self.0) as u32 == EXTPTRSXP }
    }

    /// Returns `true` if the SEXP is a function.
    pub fn is_function(&self) -> bool {
        unsafe { Rf_isFunction(self.0) == 1 }
    }

    fn is_sexp_type(&self, sexptype: SEXPTYPE) -> bool {
        match sexptype {
            savvy_ffi::INTSXP => self.is_integer(),
            savvy_ffi::REALSXP => self.is_real(),
            #[cfg(feature = "complex")]
            savvy_ffi::CPLXSXP => self.is_complex(),
            savvy_ffi::LGLSXP => self.is_logical(),
            savvy_ffi::STRSXP => self.is_string(),
            savvy_ffi::VECSXP => self.is_list(),
            savvy_ffi::EXTPTRSXP => self.is_external_pointer(),
            // cf. https://github.com/wch/r-source/blob/95ac44a87065d5b42579b621d278adc44641dcf0/src/include/Rinlinedfuns.h#L810-L815
            savvy_ffi::CLOSXP | savvy_ffi::BUILTINSXP | savvy_ffi::SPECIALSXP => self.is_function(),
            savvy_ffi::NILSXP => self.is_null(),
            _ => false,
        }
    }

    /// Returns the string representation of the SEXP type.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_human_readable_type_name(&self) -> &'static str {
        unsafe { get_human_readable_type_name(TYPEOF(self.0)) }
    }
}

unsafe fn get_human_readable_type_name(sexptype: SEXPTYPE) -> &'static str {
    unsafe {
        // TODO: replace this `R_typeToChar()` which will be introduced in R 4.4
        let c = Rf_type2char(sexptype);
        CStr::from_ptr(c).to_str().unwrap()
    }
}

macro_rules! impl_sexp_type_assert {
    ($self: ident, $sexptype: ident) => {
        if $self.is_sexp_type(savvy_ffi::$sexptype) {
            Ok(())
        } else {
            let expected = unsafe { get_human_readable_type_name(savvy_ffi::$sexptype) };
            let actual = $self.get_human_readable_type_name();
            let msg = format!("Expected {expected}, got {actual}");
            Err(crate::error::Error::UnexpectedType(msg))
        }
    };
}

impl Sexp {
    /// Returns error when the SEXP is not NULL.
    pub fn assert_null(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, NILSXP)
    }

    /// Returns error when the SEXP is not an integer vector.
    pub fn assert_integer(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, INTSXP)
    }

    /// Returns error when the SEXP is not a real vector.
    pub fn assert_real(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, REALSXP)
    }

    /// Returns error when the SEXP is not an complex pointer.
    #[cfg(feature = "complex")]
    pub fn assert_complex(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, CPLXSXP)
    }

    /// Returns error when the SEXP is not a logical vector.
    pub fn assert_logical(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, LGLSXP)
    }

    /// Returns error when the SEXP is not a string vector.
    pub fn assert_string(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, STRSXP)
    }

    /// Returns error when the SEXP is not a list.
    pub fn assert_list(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, VECSXP)
    }

    /// Returns error when the SEXP is not an external pointer.
    pub fn assert_external_pointer(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, EXTPTRSXP)
    }

    /// Returns error when the SEXP is not a function.
    pub fn assert_function(&self) -> crate::error::Result<()> {
        impl_sexp_type_assert!(self, CLOSXP)
    }
}

#[non_exhaustive]
/// A typed version of `SEXP`.
pub enum TypedSexp {
    Integer(IntegerSexp),
    Real(RealSexp),
    #[cfg(feature = "complex")]
    Complex(ComplexSexp),
    Logical(LogicalSexp),
    String(StringSexp),
    List(ListSexp),
    Null(NullSexp),
    ExternalPointer(ExternalPointerSexp),
    Function(FunctionSexp),
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
#[cfg(feature = "complex")]
into_typed_sxp!(ComplexSexp, Complex);
into_typed_sxp!(LogicalSexp, Logical);
into_typed_sxp!(StringSexp, String);
into_typed_sxp!(ListSexp, List);
into_typed_sxp!(ExternalPointerSexp, ExternalPointer);
into_typed_sxp!(FunctionSexp, Function);
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
#[cfg(feature = "complex")]
into_typed_sxp_owned!(OwnedComplexSexp, Complex);
into_typed_sxp_owned!(OwnedStringSexp, String);
into_typed_sxp_owned!(OwnedLogicalSexp, Logical);

impl From<TypedSexp> for SEXP {
    fn from(value: TypedSexp) -> Self {
        match value {
            TypedSexp::Null(_) => unsafe { savvy_ffi::R_NilValue },
            TypedSexp::Integer(sxp) => sxp.inner(),
            TypedSexp::Real(sxp) => sxp.inner(),
            #[cfg(feature = "complex")]
            TypedSexp::Complex(sxp) => sxp.inner(),
            TypedSexp::String(sxp) => sxp.inner(),
            TypedSexp::Logical(sxp) => sxp.inner(),
            TypedSexp::List(sxp) => sxp.inner(),
            TypedSexp::ExternalPointer(sxp) => sxp.inner(),
            TypedSexp::Function(sxp) => sxp.inner(),
            TypedSexp::Other(sxp) => sxp,
        }
    }
}

impl Sexp {
    /// Downcast the `SEXP` to a concrete type.
    pub fn into_typed(self) -> TypedSexp {
        let ty = unsafe { TYPEOF(self.0) };
        match ty {
            savvy_ffi::INTSXP => TypedSexp::Integer(IntegerSexp(self.0)),
            savvy_ffi::REALSXP => TypedSexp::Real(RealSexp(self.0)),
            #[cfg(feature = "complex")]
            savvy_ffi::CPLXSXP => TypedSexp::Complex(ComplexSexp(self.0)),
            savvy_ffi::LGLSXP => TypedSexp::Logical(LogicalSexp(self.0)),
            savvy_ffi::STRSXP => TypedSexp::String(StringSexp(self.0)),
            savvy_ffi::VECSXP => TypedSexp::List(ListSexp(self.0)),
            savvy_ffi::EXTPTRSXP => TypedSexp::ExternalPointer(ExternalPointerSexp(self.0)),
            // cf. https://github.com/wch/r-source/blob/95ac44a87065d5b42579b621d278adc44641dcf0/src/include/Rinlinedfuns.h#L810-L815
            savvy_ffi::CLOSXP | savvy_ffi::BUILTINSXP | savvy_ffi::SPECIALSXP => {
                TypedSexp::Function(FunctionSexp(self.0))
            }
            savvy_ffi::NILSXP => TypedSexp::Null(NullSexp),
            _ => TypedSexp::Other(self.0),
        }
    }

    /// Returns the specified attribute.
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
        let sexp = unsafe { savvy_ffi::Rf_getAttrib(self.0, attr) };

        if sexp == unsafe { savvy_ffi::R_NilValue } {
            None
        // Bravely assume the "class" attribute is always a valid STRSXP.
        } else {
            Some(crate::StringSexp(sexp).iter().collect())
        }
    }

    /// Returns the S3 class.
    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        unsafe { self.get_string_attrib_by_symbol(savvy_ffi::R_ClassSymbol) }
    }

    /// Returns the names.
    pub fn get_names(&self) -> Option<Vec<&'static str>> {
        unsafe { self.get_string_attrib_by_symbol(savvy_ffi::R_NamesSymbol) }
    }

    /// Returns the dimension.
    pub fn get_dim(&self) -> Option<&[i32]> {
        unsafe { crate::sexp::get_dim_from_sexp(&self.0) }
    }

    /// Set the input value to the specified attribute.
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

    unsafe fn set_string_attrib_by_symbol<T: AsRef<str>>(
        &mut self,
        attr: SEXP,
        values: &[T],
    ) -> crate::error::Result<()> {
        let values_sexp: OwnedStringSexp = values.try_into()?;
        unsafe {
            crate::unwind_protect(|| savvy_ffi::Rf_setAttrib(self.0, attr, values_sexp.inner()))?
        };

        Ok(())
    }

    /// Set the S3 class.
    pub fn set_class<T: AsRef<str>>(&mut self, classes: &[T]) -> crate::error::Result<()> {
        unsafe { self.set_string_attrib_by_symbol(savvy_ffi::R_ClassSymbol, classes) }
    }

    /// Set the names.
    pub fn set_names<T: AsRef<str>>(&mut self, names: &[T]) -> crate::error::Result<()> {
        unsafe { self.set_string_attrib_by_symbol(savvy_ffi::R_NamesSymbol, names) }
    }

    /// Set the dimension. `dim` can be `i32`, `usize`, or whatever
    /// numeric types that implements `TryInto<i32>`.
    pub fn set_dim<T: TryInto<i32> + Copy>(&mut self, dim: &[T]) -> crate::error::Result<()> {
        unsafe { crate::sexp::set_dim_to_sexp(self.0, dim) }
    }
}

pub(crate) unsafe fn get_dim_from_sexp(value: &SEXP) -> Option<&[i32]> {
    let dim_sexp = unsafe { Rf_getAttrib(*value, savvy_ffi::R_DimSymbol) };

    if unsafe { TYPEOF(dim_sexp) != savvy_ffi::INTSXP } {
        None
    } else {
        Some(unsafe {
            std::slice::from_raw_parts(INTEGER(dim_sexp) as _, Rf_xlength(dim_sexp) as _)
        })
    }
}

pub(crate) unsafe fn set_dim_to_sexp<T>(value: SEXP, dim: &[T]) -> crate::error::Result<()>
where
    T: TryInto<i32> + Copy,
{
    let mut dim_sexp = unsafe { OwnedIntegerSexp::new_without_init(dim.len())? };
    dim.iter()
        .enumerate()
        .for_each(|(i, &v)| dim_sexp[i] = v.try_into().unwrap_or_default());
    unsafe { savvy_ffi::Rf_setAttrib(value, savvy_ffi::R_DimSymbol, dim_sexp.inner()) };
    Ok(())
}

macro_rules! impl_common_sexp_ops {
    ($ty: ty) => {
        impl $ty {
            /// Returns the raw SEXP.
            #[inline]
            pub fn inner(&self) -> savvy_ffi::SEXP {
                self.0
            }

            /// Returns the reference to the raw SEXP. This is convenient when
            /// the lifetime is needed (e.g. returning a slice).
            #[inline]
            pub(crate) fn inner_ref(&self) -> &savvy_ffi::SEXP {
                &self.0
            }

            /// Returns the length of the SEXP.
            pub fn len(&self) -> usize {
                unsafe { savvy_ffi::Rf_xlength(self.inner()) as _ }
            }

            /// Returns `true` if the SEXP is of zero-length.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns the specified attribute.
            pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
                crate::Sexp(self.inner()).get_attrib(attr)
            }

            /// Returns the names.
            pub fn get_names(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_names()
            }

            /// Returns the S3 class.
            pub fn get_class(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_class()
            }

            /// Returns the dimension.
            pub fn get_dim(&self) -> Option<&[i32]> {
                // In order to maintain the lifetime, this cannot rely on the
                // Sexp's method. Otherwise, you'll see the "cannot return
                // reference to temporary value" error.
                unsafe { crate::sexp::get_dim_from_sexp(self.inner_ref()) }
            }
        }
    };
}

macro_rules! impl_common_sexp_ops_owned {
    ($ty: ty) => {
        impl $ty {
            /// Returns the raw SEXP.
            #[inline]
            pub fn inner(&self) -> SEXP {
                self.inner
            }

            /// Returns the reference to the raw SEXP. This is convenient when
            /// the lifetime is needed (e.g. returning a slice).
            #[inline]
            pub(crate) fn inner_ref(&self) -> &savvy_ffi::SEXP {
                &self.inner
            }

            /// Returns the length of the SEXP.
            #[inline]
            pub fn len(&self) -> usize {
                self.len
            }

            /// Returns `true` if the SEXP is of zero-length.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            /// Returns the specified attribute.
            pub fn get_attrib(&self, attr: &str) -> crate::error::Result<Option<Sexp>> {
                crate::Sexp(self.inner()).get_attrib(attr)
            }

            /// Returns the names.
            pub fn get_names(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_names()
            }

            /// Returns the S3 class.
            pub fn get_class(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.inner()).get_class()
            }

            /// Returns the dimension.
            pub fn get_dim(&self) -> Option<&[i32]> {
                // In order to maintain the lifetime, this cannot rely on the
                // Sexp's method. Otherwise, you'll see the "cannot return
                // reference to temporary value" error.
                unsafe { crate::sexp::get_dim_from_sexp(self.inner_ref()) }
            }

            /// Set the input value to the specified attribute.
            pub fn set_attrib(&mut self, attr: &str, value: Sexp) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_attrib(attr, value)
            }

            /// Set the S3 class.
            pub fn set_class<T: AsRef<str>>(&mut self, classes: &[T]) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_class(classes)
            }

            /// Set the names.
            pub fn set_names<T: AsRef<str>>(&mut self, names: &[T]) -> crate::error::Result<()> {
                crate::Sexp(self.inner()).set_names(names)
            }

            /// Set the dimension. `dim` can be `i32`, `usize`, or whatever
            /// numeric types that implements `TryInto<i32>`.
            pub fn set_dim<T: TryInto<i32> + Copy>(
                &mut self,
                dim: &[T],
            ) -> crate::error::Result<()> {
                // In order to maintain the lifetime, this cannot rely on the
                // Sexp's method. Otherwise, you'll see the "cannot return
                // reference to temporary value" error.
                unsafe { crate::sexp::set_dim_to_sexp(self.inner(), dim) }
            }
        }
    };
}

pub(crate) use impl_common_sexp_ops;
pub(crate) use impl_common_sexp_ops_owned;

use self::function::FunctionSexp;
