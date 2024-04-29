SEXP savvy_is_built_with_debug__ffi(void);
SEXP savvy_to_upper__ffi(SEXP x);
SEXP savvy_add_suffix__ffi(SEXP x, SEXP y);
SEXP savvy_times_two_int__ffi(SEXP x);
SEXP savvy_times_any_int__ffi(SEXP x, SEXP y);
SEXP savvy_times_two_numeric__ffi(SEXP x);
SEXP savvy_times_any_numeric__ffi(SEXP x, SEXP y);
SEXP savvy_flip_logical__ffi(SEXP x);
SEXP savvy_flip_logical_expert_only__ffi(SEXP x);
SEXP savvy_or_logical__ffi(SEXP x, SEXP y);
SEXP savvy_print_list__ffi(SEXP x);
SEXP savvy_list_with_no_values__ffi(void);
SEXP savvy_list_with_no_names__ffi(void);
SEXP savvy_list_with_names_and_values__ffi(void);
SEXP savvy_external_person_new__ffi(void);
SEXP savvy_get_name_external__ffi(SEXP x);
SEXP savvy_set_name_external__ffi(SEXP x, SEXP name);
SEXP savvy_init_altrep_class__ffi(DllInfo* dll_info);
SEXP savvy_altint__ffi(void);
SEXP savvy_get_class_int__ffi(SEXP x);
SEXP savvy_get_names_int__ffi(SEXP x);
SEXP savvy_get_dim_int__ffi(SEXP x);
SEXP savvy_get_attr_int__ffi(SEXP x, SEXP attr);
SEXP savvy_set_class_int__ffi(void);
SEXP savvy_set_names_int__ffi(void);
SEXP savvy_set_dim_int__ffi(void);
SEXP savvy_set_attr_int__ffi(SEXP attr, SEXP value);
SEXP savvy_new_complex__ffi(SEXP size);
SEXP savvy_first_complex__ffi(SEXP x);
SEXP savvy_abs_complex__ffi(SEXP x);
SEXP savvy_new_value_pair__ffi(SEXP a, SEXP b);
SEXP savvy_scalar_input_int__ffi(SEXP x);
SEXP savvy_scalar_input_usize__ffi(SEXP x);
SEXP savvy_scalar_input_real__ffi(SEXP x);
SEXP savvy_scalar_input_logical__ffi(SEXP x);
SEXP savvy_scalar_input_string__ffi(SEXP x);
SEXP savvy_scalar_output_int__ffi(void);
SEXP savvy_scalar_output_int2__ffi(void);
SEXP savvy_scalar_output_real__ffi(void);
SEXP savvy_scalar_output_real2__ffi(void);
SEXP savvy_scalar_output_logical__ffi(void);
SEXP savvy_scalar_output_logical2__ffi(void);
SEXP savvy_scalar_output_string__ffi(void);
SEXP savvy_scalar_output_string2__ffi(void);
SEXP savvy_scalar_output_complex__ffi(void);
SEXP savvy_scalar_output_complex2__ffi(void);
SEXP savvy_sum_int__ffi(SEXP x);
SEXP savvy_sum_real__ffi(SEXP x);
SEXP savvy_rep_int_vec__ffi(SEXP x);
SEXP savvy_rep_int_slice__ffi(SEXP x);
SEXP savvy_rep_real_vec__ffi(SEXP x);
SEXP savvy_rep_real_slice__ffi(SEXP x);
SEXP savvy_rep_bool_vec__ffi(SEXP x);
SEXP savvy_rep_bool_slice__ffi(SEXP x);
SEXP savvy_rep_str_vec__ffi(SEXP x);
SEXP savvy_rep_str_slice__ffi(SEXP x);
SEXP savvy_print_foo_enum__ffi(SEXP x);
SEXP savvy_print_foo_enum_ref__ffi(SEXP x);
SEXP savvy_foo_a__ffi(void);
SEXP savvy_safe_stop__ffi(void);
SEXP savvy_raise_error__ffi(void);
SEXP savvy_must_panic__ffi(void);
SEXP savvy_do_call__ffi(SEXP fun, SEXP args);
SEXP savvy_call_with_args__ffi(SEXP fun);
SEXP savvy_get_args__ffi(SEXP args);
SEXP savvy_new_int__ffi(SEXP size);
SEXP savvy_new_real__ffi(SEXP size);
SEXP savvy_new_bool__ffi(SEXP size);
SEXP savvy_filter_integer_odd__ffi(SEXP x);
SEXP savvy_filter_real_negative__ffi(SEXP x);
SEXP savvy_filter_complex_without_im__ffi(SEXP x);
SEXP savvy_filter_logical_duplicates__ffi(SEXP x);
SEXP savvy_filter_string_ascii__ffi(SEXP x);
SEXP savvy_fun_mod1__ffi(void);
SEXP savvy_fun_mod1_1_foo__ffi(void);

// methods and associated functions for FooEnum
SEXP savvy_FooEnum_print__ffi(SEXP self__);

// methods and associated functions for Person
SEXP savvy_Person_new__ffi(void);
SEXP savvy_Person_new2__ffi(void);
SEXP savvy_Person_new_fallible__ffi(void);
SEXP savvy_Person_another_person__ffi(SEXP self__);
SEXP savvy_Person_new_with_name__ffi(SEXP name);
SEXP savvy_Person_set_name__ffi(SEXP self__, SEXP name);
SEXP savvy_Person_name__ffi(SEXP self__);
SEXP savvy_Person_associated_function__ffi(void);

// methods and associated functions for Person2
SEXP savvy_Person2_name__ffi(SEXP self__);

// methods and associated functions for Value
SEXP savvy_Value_new__ffi(SEXP x);
SEXP savvy_Value_pair__ffi(SEXP self__, SEXP b);
SEXP savvy_Value_get__ffi(SEXP self__);
SEXP savvy_Value_get2__ffi(SEXP self__);

// methods and associated functions for ValuePair
SEXP savvy_ValuePair_new__ffi(SEXP a, SEXP b);
SEXP savvy_ValuePair_new_copy__ffi(SEXP a, SEXP b);
SEXP savvy_ValuePair_print__ffi(SEXP self__);