#include <Rinternals.h>
#include "rust/api.h"

SEXP unextendr_to_upper_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_to_upper(x);

  uintptr_t mask   = (uintptr_t) 1;
  SEXP res_aligned = (SEXP) (res & ~mask);

  if ((res & mask) == 1) {
    Rf_error("%s", CHAR(res_aligned));
  }

  return res_aligned;
}

SEXP unextendr_times_two_int_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_times_two_int(x);

  uintptr_t mask   = (uintptr_t) 1;
  SEXP res_aligned = (SEXP) (res & ~mask);

  if ((res & mask) == 1) {
    Rf_error("%s", CHAR(res_aligned));
  }

  return res_aligned;
}

SEXP unextendr_times_two_numeric_wrapper(SEXP x) {
  uintptr_t res = (uintptr_t) unextendr_times_two_numeric(x);

  uintptr_t mask   = (uintptr_t) 1;
  SEXP res_aligned = (SEXP) (res & ~mask);

  if ((res & mask) == 1) {
    Rf_error("%s", CHAR(res_aligned));
  }

  return res_aligned;
}

static const R_CallMethodDef CallEntries[] = {
    {"unextendr_to_upper",           (DL_FUNC) &unextendr_to_upper_wrapper,          1},
    {"unextendr_times_two_int",      (DL_FUNC) &unextendr_times_two_int_wrapper,     1},
    {"unextendr_times_two_numeric",  (DL_FUNC) &unextendr_times_two_numeric_wrapper, 1},
    {NULL, NULL, 0}
};

void R_init_unextendr(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
