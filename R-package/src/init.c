
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_error() directly.
            Rf_error("%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}


SEXP unextendr_to_upper_wrapper(SEXP x) {
    SEXP res = unextendr_to_upper(x);
    return handle_result(res);
}

SEXP unextendr_times_two_int_wrapper(SEXP x) {
    SEXP res = unextendr_times_two_int(x);
    return handle_result(res);
}

SEXP unextendr_times_two_numeric_wrapper(SEXP x) {
    SEXP res = unextendr_times_two_numeric(x);
    return handle_result(res);
}

SEXP unextendr_flip_logical_wrapper(SEXP x) {
    SEXP res = unextendr_flip_logical(x);
    return handle_result(res);
}

SEXP unextendr_print_list_wrapper(SEXP x) {
    SEXP res = unextendr_print_list(x);
    return handle_result(res);
}

// methods and associated functions for Person

SEXP unextendr_Person_new_wrapper() {
    SEXP res = unextendr_Person_new();
    return handle_result(res);
}

SEXP unextendr_Person_set_name_wrapper(SEXP self__, SEXP name) {
    SEXP res = unextendr_Person_set_name(self__, name);
    return handle_result(res);
}

SEXP unextendr_Person_name_wrapper(SEXP self__) {
    SEXP res = unextendr_Person_name(self__);
    return handle_result(res);
}

SEXP unextendr_Person_associated_function_wrapper() {
    SEXP res = unextendr_Person_associated_function();
    return handle_result(res);
}

static const R_CallMethodDef CallEntries[] = {
    {"unextendr_to_upper", (DL_FUNC) &unextendr_to_upper_wrapper, 1},
    {"unextendr_times_two_int", (DL_FUNC) &unextendr_times_two_int_wrapper, 1},
    {"unextendr_times_two_numeric", (DL_FUNC) &unextendr_times_two_numeric_wrapper, 1},
    {"unextendr_flip_logical", (DL_FUNC) &unextendr_flip_logical_wrapper, 1},
    {"unextendr_print_list", (DL_FUNC) &unextendr_print_list_wrapper, 1},

// methods and associated functions for Person
    {"unextendr_Person_new", (DL_FUNC) &unextendr_Person_new_wrapper, 0},
    {"unextendr_Person_set_name", (DL_FUNC) &unextendr_Person_set_name_wrapper, 2},
    {"unextendr_Person_name", (DL_FUNC) &unextendr_Person_name_wrapper, 1},
    {"unextendr_Person_associated_function", (DL_FUNC) &unextendr_Person_associated_function_wrapper, 0},
    {NULL, NULL, 0}
};

void R_init_unextendr(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}

