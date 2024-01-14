use std::ffi::CStr;

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

    pub fn get_class(&self) -> Option<Vec<&'static str>> {
        let class_sexp = unsafe { savvy_ffi::Rf_getAttrib(self.0, savvy_ffi::R_ClassSymbol) };

        if class_sexp == unsafe { savvy_ffi::R_NilValue } {
            None
        // Bravely assume the "class" attribute is always a valid STRSXP.
        } else {
            Some(crate::StringSexp(class_sexp).iter().collect())
        }
    }
}

macro_rules! impl_common_sexp_ops {
    ($ty: ty) => {
        impl $ty {
            pub fn inner(&self) -> savvy_ffi::SEXP {
                self.0
            }

            pub fn len(&self) -> usize {
                unsafe { savvy_ffi::Rf_xlength(self.inner()) as _ }
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn get_names(&self) -> Vec<&'static str> {
                let names_sexp =
                    unsafe { savvy_ffi::Rf_getAttrib(self.inner(), savvy_ffi::R_NamesSymbol) };

                if names_sexp == unsafe { savvy_ffi::R_NilValue } {
                    std::iter::repeat("").take(self.len()).collect()
                // Bravely assume the "name" attribute is always a valid STRSXP.
                } else {
                    crate::StringSexp(names_sexp).iter().collect()
                }
            }

            pub fn get_class(&self) -> Option<Vec<&'static str>> {
                crate::Sexp(self.0).get_class()
            }

            pub fn get_dim(&self) -> Option<Vec<usize>> {
                let dim_sexp =
                    unsafe { savvy_ffi::Rf_getAttrib(self.inner(), savvy_ffi::R_DimSymbol) };

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
        }
    };
}

macro_rules! impl_common_sexp_ops_owned {
    ($ty: ty) => {
        impl $ty {
            pub fn inner(&self) -> SEXP {
                self.inner
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn is_empty(&self) -> bool {
                self.len == 0
            }
        }
    };
}

pub(crate) use impl_common_sexp_ops;
pub(crate) use impl_common_sexp_ops_owned;
