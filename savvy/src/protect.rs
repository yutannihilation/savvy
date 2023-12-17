// This protection mechanism is basically a simple Rust translation of the
// implementation of cpp11.
//
// https://github.com/r-lib/cpp11/blob/main/inst/include/cpp11/protect.hpp
//
// The more explanation on this can be found on the following links:
//
// - https://github.com/RcppCore/Rcpp/issues/1081
// - https://cpp11.r-lib.org/articles/internals.html#protection
//
// However, this implementation differs from these two in several points. First,
// cpp11 stores the anchor Robj in the global options. It says it's because
//
//     It is not constructed as a static variable directly since many
//     translation units may be compiled, resulting in unrelated instances of each
//     static variable.
//
// I'm not immediately sure when this actually happens, but I think I can skip
// the consideration.
//
// Note that, extendr uses a different mechanism of using HashMap to track the
// reference counts.
//
// https://github.com/extendr/extendr/blob/master/extendr-api/src/ownership.rs
//
// I'm not sure why they chose this design, but probably it is because
//
// - for parallel-proof implementation
// - `Robj` might be cloned
//
// But, my implementation doesn't implement `Clone` trait, so I don't need to
// worry that there still exists another instance on dropping it.

use once_cell::sync::Lazy;
use savvy_ffi::{
    R_NilValue, R_PreserveObject, Rf_cons, Rf_protect, Rf_unprotect, CAR, CDR, SETCAR, SETCDR,
    SET_TAG, SEXP,
};

pub(crate) struct PreservedList(SEXP);

// cf. https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
unsafe impl Send for PreservedList {}
unsafe impl Sync for PreservedList {}

pub(crate) static PRESERVED_LIST: Lazy<PreservedList> = Lazy::new(|| unsafe {
    let r = Rf_cons(R_NilValue, R_NilValue);
    R_PreserveObject(r);
    PreservedList(r)
});

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn insert_to_preserved_list(obj: SEXP) -> SEXP {
    unsafe {
        if obj == R_NilValue {
            return R_NilValue;
        }

        // Protect the object until the operation finishes
        Rf_protect(obj);

        let preserved = PRESERVED_LIST.0;
        let token = Rf_protect(Rf_cons(preserved, CDR(preserved)));

        SET_TAG(token, obj);
        SETCDR(preserved, token);

        if CDR(token) != R_NilValue {
            SETCAR(CDR(token), token);
        }

        Rf_unprotect(2);

        token
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn release_from_preserved_list(token: SEXP) {
    unsafe {
        if token == R_NilValue {
            return;
        }

        let before = CAR(token);
        let after = CDR(token);

        SETCDR(before, after);

        if after != R_NilValue {
            SETCAR(after, before);
        }
    }
}
