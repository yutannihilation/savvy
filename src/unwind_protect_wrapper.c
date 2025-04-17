#include <setjmp.h>
#include <stdint.h>

#include <Rinternals.h>

void not_so_long_jump(void *jmpbuf, Rboolean jump) {
    if (jump == TRUE) {
        longjmp(*(jmp_buf *)jmpbuf, 1);
    }
}

SEXP unwind_protect_impl(SEXP (*fun)(void *data), void *data) {
    SEXP token = R_MakeUnwindCont();
    R_PreserveObject(token);

    jmp_buf jmpbuf;
    if (setjmp(jmpbuf)) {
        // Tag the pointer
        return (SEXP)((uintptr_t)token | 1);
    }

    SEXP res = R_UnwindProtect(fun, data, not_so_long_jump, &jmpbuf, token);

    // Comments on cpp11's code:
    //
    // R_UnwindProtect adds the result to the CAR of the continuation token,
    // which implicitly protects the result. However if there is no error and
    // R_UwindProtect does a normal exit the memory shouldn't be protected, so
    // we unset it here before returning the value ourselves.
    //
    // (ref:
    // https://github.com/r-lib/cpp11/blob/4c840c03c8d62496cdab52e0c2c0d1857925debe/inst/include/cpp11/protect.hpp#L130-L133)
    SETCAR(token, R_NilValue);

    // A token needs to be released. But, it seems cpp11 doesn't explicitly do
    // this, yet has no memory leak. I still don't understand the difference, 
    // but anyway it seems this is needed in savvy's case.
    R_ReleaseObject(token);

    return res;
}
