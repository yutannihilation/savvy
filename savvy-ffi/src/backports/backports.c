// R_altrep_class_name() and R_altrep_class_package() are unavailable before R 4.6

#include <Rversion.h>
#include <Rinternals.h>
#include <R_ext/Altrep.h>

// savvy requires R >= 4.5 (e.g. for R_getVarEx()). Without this check, building
// against an older R succeeds and the package fails at load time with an
// undefined symbol error, which is much harder to understand.
#if R_VERSION < R_Version(4, 5, 0)
#error "savvy requires R 4.5.0 or later"
#endif

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
