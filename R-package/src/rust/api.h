SEXP safe_stop(void);
SEXP raise_error(void);
SEXP to_upper(SEXP x);
SEXP add_suffix(SEXP x, SEXP y);
SEXP times_two_int(SEXP x);
SEXP times_any_int(SEXP x, SEXP y);
SEXP times_two_numeric(SEXP x);
SEXP times_any_numeric(SEXP x, SEXP y);
SEXP flip_logical(SEXP x);
SEXP or_logical(SEXP x, SEXP y);
SEXP print_list(SEXP x);
SEXP scalar_input_int(SEXP x);
SEXP scalar_input_real(SEXP x);
SEXP scalar_input_logical(SEXP x);
SEXP scalar_input_string(SEXP x);
SEXP scalar_output_int(void);
SEXP scalar_output_real(void);
SEXP scalar_output_logical(void);
SEXP scalar_output_string(void);
SEXP sum_int(SEXP x);
SEXP sum_real(SEXP x);

// methods and associated functions for Person
SEXP Person_new(void);
SEXP Person_set_name(SEXP self__, SEXP name);
SEXP Person_name(SEXP self__);
SEXP Person_associated_function(void);