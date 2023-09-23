
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

SEXP savvy_scalar_input_int_wrapper(SEXP x) {
    SEXP res = savvy_scalar_input_int(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_real_wrapper(SEXP x) {
    SEXP res = savvy_scalar_input_real(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_logical_wrapper(SEXP x) {
    SEXP res = savvy_scalar_input_logical(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_str_wrapper(SEXP x) {
    SEXP res = savvy_scalar_input_str(x);
    return handle_result(res);
}

SEXP savvy_to_upper_wrapper(SEXP x) {
    SEXP res = savvy_to_upper(x);
    return handle_result(res);
}

SEXP savvy_add_suffix_wrapper(SEXP x, SEXP y) {
    SEXP res = savvy_add_suffix(x, y);
    return handle_result(res);
}

SEXP savvy_times_two_int_wrapper(SEXP x) {
    SEXP res = savvy_times_two_int(x);
    return handle_result(res);
}

SEXP savvy_times_any_int_wrapper(SEXP x, SEXP y) {
    SEXP res = savvy_times_any_int(x, y);
    return handle_result(res);
}

SEXP savvy_times_two_numeric_wrapper(SEXP x) {
    SEXP res = savvy_times_two_numeric(x);
    return handle_result(res);
}

SEXP savvy_times_any_numeric_wrapper(SEXP x, SEXP y) {
    SEXP res = savvy_times_any_numeric(x, y);
    return handle_result(res);
}

SEXP savvy_flip_logical_wrapper(SEXP x) {
    SEXP res = savvy_flip_logical(x);
    return handle_result(res);
}

SEXP savvy_or_logical_wrapper(SEXP x, SEXP y) {
    SEXP res = savvy_or_logical(x, y);
    return handle_result(res);
}

SEXP savvy_print_list_wrapper(SEXP x) {
    SEXP res = savvy_print_list(x);
    return handle_result(res);
}


// methods and associated functions for Person
SEXP savvy_Person_new_wrapper() {
    SEXP res = savvy_Person_new();
    return handle_result(res);
}

SEXP savvy_Person_set_name_wrapper(SEXP self__, SEXP name) {
    SEXP res = savvy_Person_set_name(self__, name);
    return handle_result(res);
}

SEXP savvy_Person_name_wrapper(SEXP self__) {
    SEXP res = savvy_Person_name(self__);
    return handle_result(res);
}

SEXP savvy_Person_associated_function_wrapper() {
    SEXP res = savvy_Person_associated_function();
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_scalar_input_int", (DL_FUNC) &savvy_scalar_input_int_wrapper, 1},
    {"savvy_scalar_input_real", (DL_FUNC) &savvy_scalar_input_real_wrapper, 1},
    {"savvy_scalar_input_logical", (DL_FUNC) &savvy_scalar_input_logical_wrapper, 1},
    {"savvy_scalar_input_str", (DL_FUNC) &savvy_scalar_input_str_wrapper, 1},
    {"savvy_to_upper", (DL_FUNC) &savvy_to_upper_wrapper, 1},
    {"savvy_add_suffix", (DL_FUNC) &savvy_add_suffix_wrapper, 2},
    {"savvy_times_two_int", (DL_FUNC) &savvy_times_two_int_wrapper, 1},
    {"savvy_times_any_int", (DL_FUNC) &savvy_times_any_int_wrapper, 2},
    {"savvy_times_two_numeric", (DL_FUNC) &savvy_times_two_numeric_wrapper, 1},
    {"savvy_times_any_numeric", (DL_FUNC) &savvy_times_any_numeric_wrapper, 2},
    {"savvy_flip_logical", (DL_FUNC) &savvy_flip_logical_wrapper, 1},
    {"savvy_or_logical", (DL_FUNC) &savvy_or_logical_wrapper, 2},
    {"savvy_print_list", (DL_FUNC) &savvy_print_list_wrapper, 1},

// methods and associated functions for Person
    {"savvy_Person_new", (DL_FUNC) &savvy_Person_new_wrapper, 0},
    {"savvy_Person_set_name", (DL_FUNC) &savvy_Person_set_name_wrapper, 2},
    {"savvy_Person_name", (DL_FUNC) &savvy_Person_name_wrapper, 1},
    {"savvy_Person_associated_function", (DL_FUNC) &savvy_Person_associated_function_wrapper, 0},
    {NULL, NULL, 0}
};

void R_init_savvy(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
