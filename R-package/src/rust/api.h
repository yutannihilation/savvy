SEXP unextendr_to_upper(SEXP x);
SEXP unextendr_add_suffix(SEXP x, SEXP y);
SEXP unextendr_times_two_int(SEXP x);
SEXP unextendr_times_any_int(SEXP x, SEXP y);
SEXP unextendr_times_two_numeric(SEXP x);
SEXP unextendr_times_any_numeric(SEXP x, SEXP y);
SEXP unextendr_flip_logical(SEXP x);
SEXP unextendr_or_logical(SEXP x, SEXP y);
SEXP unextendr_print_list(SEXP x);

// methods and associated functions for Person
SEXP unextendr_Person_new();
SEXP unextendr_Person_set_name(SEXP self__, SEXP name);
SEXP unextendr_Person_name(SEXP self__);
SEXP unextendr_Person_associated_function();
