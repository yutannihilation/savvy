// This file is based on the implementation of extendr and cpp11.
//
// extendr:
// https://github.com/extendr/extendr/blob/master/extendr-api/src/ownership.rs
//
// cpp11:
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
// Also, extendr manages reference count by itself. I guess this is because it
// aims for parallel-proof implementation. But, I don't think it's a good idea
// to call R API parallelly anyway, so I also decided not to consider it.

use libR_sys::{
    R_NilValue, R_PreserveObject, Rf_cons, Rf_protect, Rf_unprotect, CAR, CDR, SETCAR, SETCDR,
    SET_TAG, SEXP,
};
use once_cell::sync::Lazy;

pub(crate) struct ReservedList(SEXP);

// cf. https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
unsafe impl Send for ReservedList {}
unsafe impl Sync for ReservedList {}

pub(crate) static PRESERVED_LIST: Lazy<ReservedList> = Lazy::new(|| unsafe {
    let r = Rf_cons(R_NilValue, R_NilValue);
    R_PreserveObject(r);
    ReservedList(r)
});

impl ReservedList {
    // Note that, since PRESERVED_LIST is initialized lazily, .insert() might
    // invoke several R API calls at the first time. So, such code like
    //
    //     let out = Rf_allocVector(STRSXP, len);
    //     let token = PRESERVED_LIST.insert(out);
    //
    // is unsafe in that `out` might be GC-ed before `insert()`. I actually see
    // the R session crashes. To prevent this, we have two options:
    //
    // First idea is to simply PROTECT(). However, this is not very efficient
    // because the object gets PROTECT()-ed inside `insert()` anyway.
    //
    //     let out = Rf_protect(Rf_allocVector(STRSXP, len));
    //     let token = PRESERVED_LIST.insert(out);
    //     Rf_unprotect(1);
    //
    // Second idea is to make sure the PRESERVED_LIST is initialized. For
    // example, we can call a function like `init_preserve_list` in .onLoad().
    // But, probably, better idea is to make this function not directly belong
    // to `ReservedList`.
    pub fn insert(&self, obj: SEXP) -> SEXP {
        unsafe {
            if (obj == R_NilValue) {
                return R_NilValue;
            }

            // Protect the object until the operation finishes
            Rf_protect(obj);

            let token = Rf_protect(Rf_cons(PRESERVED_LIST.0, CDR(PRESERVED_LIST.0)));

            SET_TAG(token, obj);
            SETCDR(PRESERVED_LIST.0, token);

            if (CDR(token) != R_NilValue) {
                SETCAR(CDR(token), token);
            }

            Rf_unprotect(2);

            token
        }
    }

    pub fn release(&self, token: SEXP) {
        unsafe {
            if (token == R_NilValue) {
                return;
            }

            let before = CAR(token);
            let after = CDR(token);

            SETCDR(before, after);

            if (after != R_NilValue) {
                SETCAR(after, before);
            }
        }
    }

    pub fn inner(&self) -> SEXP {
        self.0
    }
}
