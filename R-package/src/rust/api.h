SEXP unextendr_to_upper(SEXP x);

SEXP unextendr_times_two_int(SEXP x);

SEXP unextendr_times_two_numeric(SEXP x);

SEXP unextendr_flip_logical(SEXP x);

extern SEXP unwind_protect_impl(SEXP (*fun)(void *data), void *data);
