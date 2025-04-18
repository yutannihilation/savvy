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
    PROTECT(token);

    jmp_buf jmpbuf;
    if (setjmp(jmpbuf)) {
        // Tag the pointer
        return (SEXP)((uintptr_t)token | 1);
    }

    SEXP res = R_UnwindProtect(fun, data, not_so_long_jump, &jmpbuf, token);

    UNPROTECT(1);
    return res;
}
