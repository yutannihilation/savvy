SEXP savvy_to_upper(SEXP x);
SEXP savvy_add_suffix(SEXP x, SEXP y);
SEXP savvy_times_two_int(SEXP x);
SEXP savvy_times_any_int(SEXP x, SEXP y);
SEXP savvy_times_two_numeric(SEXP x);
SEXP savvy_times_any_numeric(SEXP x, SEXP y);
SEXP savvy_flip_logical(SEXP x);
SEXP savvy_or_logical(SEXP x, SEXP y);
SEXP savvy_print_list(SEXP x);

// methods and associated functions for Person
SEXP savvy_Person_new();
SEXP savvy_Person_set_name(SEXP self__, SEXP name);
SEXP savvy_Person_name(SEXP self__);
SEXP savvy_Person_associated_function();