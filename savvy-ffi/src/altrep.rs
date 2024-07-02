#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_void};

use crate::{R_xlen_t, Rboolean, SEXP, SEXPTYPE};

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

    pub fn ALTREP(x: SEXP) -> c_int;
    pub fn ALTREP_CLASS(x: SEXP) -> SEXP;
    pub fn R_altrep_inherits(x: SEXP, arg1: R_altrep_class_t) -> Rboolean;
    pub fn R_new_altrep(aclass: R_altrep_class_t, data1: SEXP, data2: SEXP) -> SEXP;
    pub fn R_altrep_data1(x: SEXP) -> SEXP;
    pub fn R_altrep_data2(x: SEXP) -> SEXP;
    pub fn R_set_altrep_data1(x: SEXP, v: SEXP);
    pub fn R_set_altrep_data2(x: SEXP, v: SEXP);
}

// general

extern "C" {
    pub fn R_set_altrep_Unserialize_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    );
    pub fn R_set_altrep_Serialized_state_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> SEXP>,
    );
    pub fn R_set_altrep_Duplicate_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
    pub fn R_set_altrep_Coerce_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>,
    );
    pub fn R_set_altrep_Inspect_method(
        cls: R_altrep_class_t,
        fun: Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: c_int,
                arg3: c_int,
                arg4: c_int,
                arg5: Option<
                    unsafe extern "C" fn(arg1: SEXP, arg2: c_int, arg3: c_int, arg4: c_int),
                >,
            ) -> Rboolean,
        >,
    );
    pub fn R_set_altrep_Length_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> R_xlen_t>,
    );
}

// vector common

extern "C" {
    pub fn R_set_altvec_Dataptr_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> *mut c_void>,
    );
    pub fn R_set_altvec_Dataptr_or_null_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> *const c_void>,
    );

}

// integer

extern "C" {
    pub fn R_make_altinteger_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altinteger_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> c_int>,
    );
    pub fn R_set_altinteger_Get_region_method(
        cls: R_altrep_class_t,
        fun: Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: R_xlen_t,
                arg3: R_xlen_t,
                arg4: *mut c_int,
            ) -> R_xlen_t,
        >,
    );
    pub fn R_set_altinteger_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altinteger_No_NA_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altinteger_Sum_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
    pub fn R_set_altinteger_Min_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
    pub fn R_set_altinteger_Max_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
}

// real

extern "C" {
    pub fn R_make_altreal_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altreal_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> f64>,
    );
    pub fn R_set_altreal_Get_region_method(
        cls: R_altrep_class_t,
        fun: Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: R_xlen_t,
                arg3: R_xlen_t,
                arg4: *mut f64,
            ) -> R_xlen_t,
        >,
    );
    pub fn R_set_altreal_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altreal_No_NA_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altreal_Sum_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
    pub fn R_set_altreal_Min_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
    pub fn R_set_altreal_Max_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
}

// logical

extern "C" {
    pub fn R_make_altlogical_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altlogical_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> c_int>,
    );
    pub fn R_set_altlogical_Get_region_method(
        cls: R_altrep_class_t,
        fun: Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: R_xlen_t,
                arg3: R_xlen_t,
                arg4: *mut c_int,
            ) -> R_xlen_t,
        >,
    );
    pub fn R_set_altlogical_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altlogical_No_NA_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altlogical_Sum_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>,
    );
}

// raw
extern "C" {
    pub fn R_make_altraw_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altraw_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>,
    );
    pub fn R_set_altraw_Get_region_method(
        cls: R_altrep_class_t,
        fun: Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: R_xlen_t,
                arg3: R_xlen_t,
                arg4: *mut u8,
            ) -> R_xlen_t,
        >,
    );
}

// string

extern "C" {
    pub fn R_make_altstring_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altstring_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>,
    );
    pub fn R_set_altstring_Set_elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>,
    );
    pub fn R_set_altstring_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
    pub fn R_set_altstring_No_NA_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP) -> c_int>,
    );
}

// list

extern "C" {
    pub fn R_make_altlist_class(
        cname: *const c_char,
        pname: *const c_char,
        info: *mut crate::DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_set_altlist_Elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>,
    );
    pub fn R_set_altlist_Set_elt_method(
        cls: R_altrep_class_t,
        fun: Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>,
    );
}
