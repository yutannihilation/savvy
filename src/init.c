#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t) 1;

SEXP unextendr_to_upper_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_to_upper(x);

  if ((res & TAGGED_POINTER_MASK) == 1) {
    SEXP res_aligned = (SEXP) (res & ~TAGGED_POINTER_MASK);
    Rf_error("%s", CHAR(res_aligned));
  }

  return (SEXP) res;
}

SEXP unextendr_times_two_int_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_times_two_int(x);

  if ((res & TAGGED_POINTER_MASK) == 1) {
    SEXP res_aligned = (SEXP) (res & ~TAGGED_POINTER_MASK);
    Rf_error("%s", CHAR(res_aligned));
  }

  return (SEXP) res;
}

SEXP unextendr_times_two_numeric_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_times_two_numeric(x);

  if ((res & TAGGED_POINTER_MASK) == 1) {
    SEXP res_aligned = (SEXP) (res & ~TAGGED_POINTER_MASK);
    Rf_error("%s", CHAR(res_aligned));
  }

  return (SEXP) res;
}

SEXP unextendr_flip_logical_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_flip_logical(x);

  if ((res & TAGGED_POINTER_MASK) == 1) {
    SEXP res_aligned = (SEXP) (res & ~TAGGED_POINTER_MASK);
    Rf_error("%s", CHAR(res_aligned));
  }

  return (SEXP) res;
}

static const R_CallMethodDef CallEntries[] = {
    {"unextendr_to_upper",           (DL_FUNC) &unextendr_to_upper_wrapper,          1},
    {"unextendr_times_two_int",      (DL_FUNC) &unextendr_times_two_int_wrapper,     1},
    {"unextendr_times_two_numeric",  (DL_FUNC) &unextendr_times_two_numeric_wrapper, 1},
    {"unextendr_flip_logical",       (DL_FUNC) &unextendr_flip_logical_wrapper,      1},
    {NULL, NULL, 0}
};

void R_init_unextendr(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
