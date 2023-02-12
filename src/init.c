#include <Rinternals.h>
#include "rust/api.h"

static const R_CallMethodDef CallEntries[] = {
    {"unextendr_to_upper",       (DL_FUNC) &unextendr_to_upper,       1},
//    {"unextendr_preserve_list",  (DL_FUNC) &unextendr_preserve_list,  0},
    {NULL, NULL, 0}
};

void R_init_unextendr(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
