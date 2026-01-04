// R_altrep_class_name() and R_altrep_class_package() are unavailable before R 4.6

#include <Rversion.h>
#include <Rinternals.h>
#include <R_ext/Altrep.h>

#if R_VERSION < R_Version(4, 6, 0)
SEXP R_altrep_class_name(SEXP x)
{
    return ALTREP(x) ? CAR(ATTRIB(ALTREP_CLASS(x))) : R_NilValue;
}
SEXP R_altrep_class_package(SEXP x)
{
    return ALTREP(x) ? CADR(ATTRIB(ALTREP_CLASS(x))) : R_NilValue;
}
#endif