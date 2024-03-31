SEXP get_class_int(SEXP x);
SEXP get_names_int(SEXP x);
SEXP get_dim_int(SEXP x);
SEXP get_attr_int(SEXP x, SEXP attr);
SEXP set_class_int(void);
SEXP set_names_int(void);
SEXP set_dim_int(void);
SEXP set_attr_int(SEXP attr, SEXP value);
SEXP new_complex(SEXP size);
SEXP first_complex(SEXP x);
SEXP abs_complex(SEXP x);
SEXP new_value_pair(SEXP a, SEXP b);
SEXP scalar_input_int(SEXP x);
SEXP scalar_input_usize(SEXP x);
SEXP scalar_input_real(SEXP x);
SEXP scalar_input_logical(SEXP x);
SEXP scalar_input_string(SEXP x);
SEXP scalar_output_int(void);
SEXP scalar_output_int2(void);
SEXP scalar_output_real(void);
SEXP scalar_output_real2(void);
SEXP scalar_output_logical(void);
SEXP scalar_output_logical2(void);
SEXP scalar_output_string(void);
SEXP scalar_output_string2(void);
SEXP scalar_output_complex(void);
SEXP scalar_output_complex2(void);
SEXP sum_int(SEXP x);
SEXP sum_real(SEXP x);
SEXP rep_int_vec(SEXP x);
SEXP rep_int_slice(SEXP x);
SEXP rep_real_vec(SEXP x);
SEXP rep_real_slice(SEXP x);
SEXP rep_bool_vec(SEXP x);
SEXP rep_bool_slice(SEXP x);
SEXP rep_str_vec(SEXP x);
SEXP rep_str_slice(SEXP x);
SEXP safe_stop(void);
SEXP raise_error(void);
SEXP do_call(SEXP fun, SEXP args);
SEXP call_with_args(SEXP fun);
SEXP get_args(SEXP args);
SEXP new_int(SEXP size);
SEXP new_real(SEXP size);
SEXP new_bool(SEXP size);
SEXP to_upper(SEXP x);
SEXP add_suffix(SEXP x, SEXP y);
SEXP times_two_int(SEXP x);
SEXP times_any_int(SEXP x, SEXP y);
SEXP times_two_numeric(SEXP x);
SEXP times_any_numeric(SEXP x, SEXP y);
SEXP flip_logical(SEXP x);
SEXP flip_logical_expert_only(SEXP x);
SEXP or_logical(SEXP x, SEXP y);
SEXP print_list(SEXP x);
SEXP list_with_no_values(void);
SEXP list_with_no_names(void);
SEXP list_with_names_and_values(void);
SEXP external_person_new(void);
SEXP get_name_external(SEXP x);
SEXP set_name_external(SEXP x, SEXP name);
SEXP filter_integer_odd(SEXP x);
SEXP filter_real_negative(SEXP x);
SEXP filter_complex_without_im(SEXP x);
SEXP filter_logical_duplicates(SEXP x);
SEXP filter_string_ascii(SEXP x);

// methods and associated functions for Value
SEXP Value_new(SEXP x);
SEXP Value_pair(SEXP self__, SEXP b);
SEXP Value_get(SEXP self__);

// methods and associated functions for ValuePair
SEXP ValuePair_new(SEXP a, SEXP b);
SEXP ValuePair_new_copy(SEXP a, SEXP b);
SEXP ValuePair_print(SEXP self__);

// methods and associated functions for Person
SEXP Person_new(void);
SEXP Person_new2(void);
SEXP Person_new_fallible(void);
SEXP Person_another_person(SEXP self__);
SEXP Person_new_with_name(SEXP name);
SEXP Person_set_name(SEXP self__, SEXP name);
SEXP Person_name(SEXP self__);
SEXP Person_associated_function(void);

// methods and associated functions for Person2
SEXP Person2_name(SEXP self__);