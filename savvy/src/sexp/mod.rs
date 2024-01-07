use std::ffi::CStr;

use savvy_ffi::{
    Rf_isInteger, Rf_isLogical, Rf_isReal, Rf_isString, Rf_type2char, EXTPTRSXP, SEXP, TYPEOF,
    VECSXP,
};

use crate::{
    IntegerSxp, ListSxp, LogicalSxp, NullSxp, OwnedIntegerSxp, OwnedLogicalSxp, OwnedRealSxp,
    OwnedStringSxp, RealSxp, StringSxp,
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

pub struct Sxp(pub SEXP);

impl Sxp {
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

pub enum TypedSxp {
    Integer(IntegerSxp),
    Real(RealSxp),
    String(StringSxp),
    Logical(LogicalSxp),
    List(ListSxp),
    Null(NullSxp),
    Other(SEXP),
}

macro_rules! into_typed_sxp {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for TypedSxp {
            fn from(value: $ty) -> Self {
                TypedSxp::$variant(value)
            }
        }
    };
}

into_typed_sxp!(IntegerSxp, Integer);
into_typed_sxp!(RealSxp, Real);
into_typed_sxp!(StringSxp, String);
into_typed_sxp!(LogicalSxp, Logical);
into_typed_sxp!(ListSxp, List);
into_typed_sxp!(NullSxp, Null);

macro_rules! into_typed_sxp_owned {
    ($ty: ty, $variant: ident) => {
        impl From<$ty> for TypedSxp {
            fn from(value: $ty) -> Self {
                TypedSxp::$variant(value.as_read_only())
            }
        }
    };
}

into_typed_sxp_owned!(OwnedIntegerSxp, Integer);
into_typed_sxp_owned!(OwnedRealSxp, Real);
into_typed_sxp_owned!(OwnedStringSxp, String);
into_typed_sxp_owned!(OwnedLogicalSxp, Logical);

impl From<TypedSxp> for SEXP {
    fn from(value: TypedSxp) -> Self {
        match value {
            TypedSxp::Null(e) => e.into(),
            TypedSxp::Integer(e) => e.inner(),
            TypedSxp::Real(e) => e.inner(),
            TypedSxp::String(e) => e.inner(),
            TypedSxp::Logical(e) => e.inner(),
            TypedSxp::List(e) => e.inner(),
            TypedSxp::Other(e) => e,
        }
    }
}

impl Sxp {
    pub fn into_typed(self) -> TypedSxp {
        let ty = unsafe { TYPEOF(self.0) };
        match ty as u32 {
            savvy_ffi::INTSXP => TypedSxp::Integer(IntegerSxp(self.0)),
            savvy_ffi::REALSXP => TypedSxp::Real(RealSxp(self.0)),
            savvy_ffi::STRSXP => TypedSxp::String(StringSxp(self.0)),
            savvy_ffi::LGLSXP => TypedSxp::Logical(LogicalSxp(self.0)),
            savvy_ffi::VECSXP => TypedSxp::List(ListSxp(self.0)),
            savvy_ffi::NILSXP => TypedSxp::Null(NullSxp),
            _ => TypedSxp::Other(self.0),
        }
    }
}
