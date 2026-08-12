// R_altrep_class_name() and R_altrep_class_package() are unavailable before R 4.6

#include <Rversion.h>
#include <Rinternals.h>
#include <R_ext/Altrep.h>

// savvy requires R >= 4.2 (e.g. for R_existsVarInFrame()). Without this check,
// building against an older R succeeds and the package fails at load time with
// an undefined symbol error, which is much harder to understand.
#if R_VERSION < R_Version(4, 2, 0)
#error "savvy requires R 4.2.0 or later"
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

#if R_VERSION < R_Version(4, 5, 0)
const void *VECTOR_PTR_RO(SEXP x)
{
    return DATAPTR_RO(x);
}

SEXP R_getVarEx(SEXP sym, SEXP rho, Rboolean inherits, SEXP ifnotfound)
{
    SEXP val;

    if (inherits) {
        val = Rf_findVar(sym, rho);
        if (val == R_UnboundValue)
            return ifnotfound;
    } else {
        if (R_existsVarInFrame(rho, sym) != TRUE)
            return ifnotfound;
        // Note: unlike the inherits case, there's no API to get the value
        // without forcing it, so let Rf_eval() do the lookup and the forcing.
        return Rf_eval(sym, rho);
    }

    // A promise needs to be forced to get the value.
    if (TYPEOF(val) == PROMSXP) {
        PROTECT(val);
        val = Rf_eval(val, rho);
        UNPROTECT(1);
    }

    return val;
}
#endif
