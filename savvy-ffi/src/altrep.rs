#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{R_xlen_t, Rboolean, SEXP};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DllInfo {
    _unused: [u8; 0],
}
pub type DllInfo = _DllInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_altrep_class_t {
    pub ptr: SEXP,
}

// I'm not fully confident, but R_altrep_class_t should be thread safe in the
// sense that this can be set during the initialization.
unsafe impl Send for R_altrep_class_t {}
unsafe impl Sync for R_altrep_class_t {}

extern "C" {
    // Note: this function is not limited to ALTREP, but this is placed here
    // because it's currently needed only for ALTREP.
    pub fn MARK_NOT_MUTABLE(x: SEXP);

    pub fn ALTREP(x: SEXP) -> ::std::os::raw::c_int;
    pub fn ALTREP_CLASS(x: SEXP) -> SEXP;
    pub fn R_new_altrep(aclass: R_altrep_class_t, data1: SEXP, data2: SEXP) -> SEXP;
    pub fn R_altrep_data1(x: SEXP) -> SEXP;
    pub fn R_altrep_data2(x: SEXP) -> SEXP;
    pub fn R_set_altrep_data1(x: SEXP, v: SEXP);
    pub fn R_set_altrep_data2(x: SEXP, v: SEXP);
}

// general

pub type R_altrep_UnserializeEX_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: SEXP,
        arg3: SEXP,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> SEXP,
>;
pub type R_altrep_Unserialize_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>;
pub type R_altrep_Serialized_state_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> SEXP>;
pub type R_altrep_DuplicateEX_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Duplicate_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Coerce_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: ::std::os::raw::c_int) -> SEXP>;
pub type R_altrep_Inspect_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ),
        >,
    ) -> Rboolean,
>;
pub type R_altrep_Length_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> R_xlen_t>;
extern "C" {
    pub fn R_set_altrep_UnserializeEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_UnserializeEX_method_t,
    );
    pub fn R_set_altrep_Unserialize_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Unserialize_method_t,
    );
    pub fn R_set_altrep_Serialized_state_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Serialized_state_method_t,
    );
    pub fn R_set_altrep_DuplicateEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_DuplicateEX_method_t,
    );
    pub fn R_set_altrep_Duplicate_method(cls: R_altrep_class_t, fun: R_altrep_Duplicate_method_t);
    pub fn R_set_altrep_Coerce_method(cls: R_altrep_class_t, fun: R_altrep_Coerce_method_t);
    pub fn R_set_altrep_Inspect_method(cls: R_altrep_class_t, fun: R_altrep_Inspect_method_t);
    pub fn R_set_altrep_Length_method(cls: R_altrep_class_t, fun: R_altrep_Length_method_t);
}

// integer

pub type R_altinteger_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
pub type R_altinteger_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
pub type R_altinteger_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;

extern "C" {
    pub fn R_make_altinteger_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altinteger_Elt_method(cls: R_altrep_class_t, fun: R_altinteger_Elt_method_t);
    pub fn R_set_altinteger_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Get_region_method_t,
    );
    pub fn R_set_altinteger_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Is_sorted_method_t,
    );
    pub fn R_set_altinteger_No_NA_method(cls: R_altrep_class_t, fun: R_altinteger_No_NA_method_t);
    pub fn R_set_altinteger_Sum_method(cls: R_altrep_class_t, fun: R_altinteger_Sum_method_t);
    pub fn R_set_altinteger_Min_method(cls: R_altrep_class_t, fun: R_altinteger_Min_method_t);
    pub fn R_set_altinteger_Max_method(cls: R_altrep_class_t, fun: R_altinteger_Max_method_t);
}
