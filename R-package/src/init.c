
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

SEXP scalar_input_int__impl(SEXP x) {
    SEXP res = scalar_input_int(x);
    return handle_result(res);
}

SEXP scalar_input_real__impl(SEXP x) {
    SEXP res = scalar_input_real(x);
    return handle_result(res);
}

SEXP scalar_input_logical__impl(SEXP x) {
    SEXP res = scalar_input_logical(x);
    return handle_result(res);
}

SEXP scalar_input_str__impl(SEXP x) {
    SEXP res = scalar_input_str(x);
    return handle_result(res);
}

SEXP to_upper__impl(SEXP x) {
    SEXP res = to_upper(x);
    return handle_result(res);
}

SEXP add_suffix__impl(SEXP x, SEXP y) {
    SEXP res = add_suffix(x, y);
    return handle_result(res);
}

SEXP times_two_int__impl(SEXP x) {
    SEXP res = times_two_int(x);
    return handle_result(res);
}

SEXP times_any_int__impl(SEXP x, SEXP y) {
    SEXP res = times_any_int(x, y);
    return handle_result(res);
}

SEXP times_two_numeric__impl(SEXP x) {
    SEXP res = times_two_numeric(x);
    return handle_result(res);
}

SEXP times_any_numeric__impl(SEXP x, SEXP y) {
    SEXP res = times_any_numeric(x, y);
    return handle_result(res);
}

SEXP flip_logical__impl(SEXP x) {
    SEXP res = flip_logical(x);
    return handle_result(res);
}

SEXP or_logical__impl(SEXP x, SEXP y) {
    SEXP res = or_logical(x, y);
    return handle_result(res);
}

SEXP print_list__impl(SEXP x) {
    SEXP res = print_list(x);
    return handle_result(res);
}


// methods and associated functions for Person
SEXP Person_new__impl() {
    SEXP res = Person_new();
    return handle_result(res);
}

SEXP Person_set_name__impl(SEXP self__, SEXP name) {
    SEXP res = Person_set_name(self__, name);
    return handle_result(res);
}

SEXP Person_name__impl(SEXP self__) {
    SEXP res = Person_name(self__);
    return handle_result(res);
}

SEXP Person_associated_function__impl() {
    SEXP res = Person_associated_function();
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"scalar_input_int__impl", (DL_FUNC) &scalar_input_int__impl, 1},
    {"scalar_input_real__impl", (DL_FUNC) &scalar_input_real__impl, 1},
    {"scalar_input_logical__impl", (DL_FUNC) &scalar_input_logical__impl, 1},
    {"scalar_input_str__impl", (DL_FUNC) &scalar_input_str__impl, 1},
    {"to_upper__impl", (DL_FUNC) &to_upper__impl, 1},
    {"add_suffix__impl", (DL_FUNC) &add_suffix__impl, 2},
    {"times_two_int__impl", (DL_FUNC) &times_two_int__impl, 1},
    {"times_any_int__impl", (DL_FUNC) &times_any_int__impl, 2},
    {"times_two_numeric__impl", (DL_FUNC) &times_two_numeric__impl, 1},
    {"times_any_numeric__impl", (DL_FUNC) &times_any_numeric__impl, 2},
    {"flip_logical__impl", (DL_FUNC) &flip_logical__impl, 1},
    {"or_logical__impl", (DL_FUNC) &or_logical__impl, 2},
    {"print_list__impl", (DL_FUNC) &print_list__impl, 1},

// methods and associated functions for Person
    {"Person_new__impl", (DL_FUNC) &Person_new__impl, 0},
    {"Person_set_name__impl", (DL_FUNC) &Person_set_name__impl, 2},
    {"Person_name__impl", (DL_FUNC) &Person_name__impl, 1},
    {"Person_associated_function__impl", (DL_FUNC) &Person_associated_function__impl, 0},
    {NULL, NULL, 0}
};

void R_init_savvy(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
