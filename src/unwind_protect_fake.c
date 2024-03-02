// This is just a fake function in case Rinternals.h is not available (e.g.,
// cargo test)

typedef void *SEXP;

SEXP unwind_protect_impl(SEXP (*fun)(void *data), void *data) {
    return fun(data);
}
