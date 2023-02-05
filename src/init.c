#include <Rinternals.h>
#include "rust/api.h"

static const R_CallMethodDef CallEntries[] = {
    {"unextendr_string", (DL_FUNC) &unextendr_string, 1},
    {NULL, NULL, 0}
};

void R_init_unextendr(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
