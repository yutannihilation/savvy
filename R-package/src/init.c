
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP to_upper__impl(SEXP x) {
    SEXP res = to_upper(x);
    return handle_result(res);
}

SEXP add_suffix__impl(SEXP x, SEXP y) {
    SEXP res = add_suffix(x, y);
    return handle_result(res);
}

SEXP times_two_int__impl(SEXP x) {
    SEXP res = times_two_int(x);
    return handle_result(res);
}

SEXP times_any_int__impl(SEXP x, SEXP y) {
    SEXP res = times_any_int(x, y);
    return handle_result(res);
}

SEXP times_two_numeric__impl(SEXP x) {
    SEXP res = times_two_numeric(x);
    return handle_result(res);
}

SEXP times_any_numeric__impl(SEXP x, SEXP y) {
    SEXP res = times_any_numeric(x, y);
    return handle_result(res);
}

SEXP flip_logical__impl(SEXP x) {
    SEXP res = flip_logical(x);
    return handle_result(res);
}

SEXP flip_logical_expert_only__impl(SEXP x) {
    SEXP res = flip_logical_expert_only(x);
    return handle_result(res);
}

SEXP or_logical__impl(SEXP x, SEXP y) {
    SEXP res = or_logical(x, y);
    return handle_result(res);
}

SEXP print_list__impl(SEXP x) {
    SEXP res = print_list(x);
    return handle_result(res);
}

SEXP list_with_no_values__impl(void) {
    SEXP res = list_with_no_values();
    return handle_result(res);
}

SEXP list_with_no_names__impl(void) {
    SEXP res = list_with_no_names();
    return handle_result(res);
}

SEXP list_with_names_and_values__impl(void) {
    SEXP res = list_with_names_and_values();
    return handle_result(res);
}

SEXP external_person_new__impl(void) {
    SEXP res = external_person_new();
    return handle_result(res);
}

SEXP get_name_external__impl(SEXP x) {
    SEXP res = get_name_external(x);
    return handle_result(res);
}

SEXP set_name_external__impl(SEXP x, SEXP name) {
    SEXP res = set_name_external(x, name);
    return handle_result(res);
}

SEXP get_class_int__impl(SEXP x) {
    SEXP res = get_class_int(x);
    return handle_result(res);
}

SEXP get_names_int__impl(SEXP x) {
    SEXP res = get_names_int(x);
    return handle_result(res);
}

SEXP get_dim_int__impl(SEXP x) {
    SEXP res = get_dim_int(x);
    return handle_result(res);
}

SEXP get_attr_int__impl(SEXP x, SEXP attr) {
    SEXP res = get_attr_int(x, attr);
    return handle_result(res);
}

SEXP set_class_int__impl(void) {
    SEXP res = set_class_int();
    return handle_result(res);
}

SEXP set_names_int__impl(void) {
    SEXP res = set_names_int();
    return handle_result(res);
}

SEXP set_dim_int__impl(void) {
    SEXP res = set_dim_int();
    return handle_result(res);
}

SEXP set_attr_int__impl(SEXP attr, SEXP value) {
    SEXP res = set_attr_int(attr, value);
    return handle_result(res);
}

SEXP scalar_input_int__impl(SEXP x) {
    SEXP res = scalar_input_int(x);
    return handle_result(res);
}

SEXP scalar_input_usize__impl(SEXP x) {
    SEXP res = scalar_input_usize(x);
    return handle_result(res);
}

SEXP scalar_input_real__impl(SEXP x) {
    SEXP res = scalar_input_real(x);
    return handle_result(res);
}

SEXP scalar_input_logical__impl(SEXP x) {
    SEXP res = scalar_input_logical(x);
    return handle_result(res);
}

SEXP scalar_input_string__impl(SEXP x) {
    SEXP res = scalar_input_string(x);
    return handle_result(res);
}

SEXP scalar_output_int__impl(void) {
    SEXP res = scalar_output_int();
    return handle_result(res);
}

SEXP scalar_output_int2__impl(void) {
    SEXP res = scalar_output_int2();
    return handle_result(res);
}

SEXP scalar_output_real__impl(void) {
    SEXP res = scalar_output_real();
    return handle_result(res);
}

SEXP scalar_output_real2__impl(void) {
    SEXP res = scalar_output_real2();
    return handle_result(res);
}

SEXP scalar_output_logical__impl(void) {
    SEXP res = scalar_output_logical();
    return handle_result(res);
}

SEXP scalar_output_logical2__impl(void) {
    SEXP res = scalar_output_logical2();
    return handle_result(res);
}

SEXP scalar_output_string__impl(void) {
    SEXP res = scalar_output_string();
    return handle_result(res);
}

SEXP scalar_output_string2__impl(void) {
    SEXP res = scalar_output_string2();
    return handle_result(res);
}

SEXP scalar_output_complex__impl(void) {
    SEXP res = scalar_output_complex();
    return handle_result(res);
}

SEXP scalar_output_complex2__impl(void) {
    SEXP res = scalar_output_complex2();
    return handle_result(res);
}

SEXP sum_int__impl(SEXP x) {
    SEXP res = sum_int(x);
    return handle_result(res);
}

SEXP sum_real__impl(SEXP x) {
    SEXP res = sum_real(x);
    return handle_result(res);
}

SEXP rep_int_vec__impl(SEXP x) {
    SEXP res = rep_int_vec(x);
    return handle_result(res);
}

SEXP rep_int_slice__impl(SEXP x) {
    SEXP res = rep_int_slice(x);
    return handle_result(res);
}

SEXP rep_real_vec__impl(SEXP x) {
    SEXP res = rep_real_vec(x);
    return handle_result(res);
}

SEXP rep_real_slice__impl(SEXP x) {
    SEXP res = rep_real_slice(x);
    return handle_result(res);
}

SEXP rep_bool_vec__impl(SEXP x) {
    SEXP res = rep_bool_vec(x);
    return handle_result(res);
}

SEXP rep_bool_slice__impl(SEXP x) {
    SEXP res = rep_bool_slice(x);
    return handle_result(res);
}

SEXP rep_str_vec__impl(SEXP x) {
    SEXP res = rep_str_vec(x);
    return handle_result(res);
}

SEXP rep_str_slice__impl(SEXP x) {
    SEXP res = rep_str_slice(x);
    return handle_result(res);
}

SEXP safe_stop__impl(void) {
    SEXP res = safe_stop();
    return handle_result(res);
}

SEXP raise_error__impl(void) {
    SEXP res = raise_error();
    return handle_result(res);
}

SEXP new_int__impl(SEXP size) {
    SEXP res = new_int(size);
    return handle_result(res);
}

SEXP new_real__impl(SEXP size) {
    SEXP res = new_real(size);
    return handle_result(res);
}

SEXP new_bool__impl(SEXP size) {
    SEXP res = new_bool(size);
    return handle_result(res);
}

SEXP do_call__impl(SEXP fun, SEXP args) {
    SEXP res = do_call(fun, args);
    return handle_result(res);
}

SEXP call_with_args__impl(SEXP fun) {
    SEXP res = call_with_args(fun);
    return handle_result(res);
}

SEXP get_args__impl(SEXP args) {
    SEXP res = get_args(args);
    return handle_result(res);
}

SEXP new_complex__impl(SEXP size) {
    SEXP res = new_complex(size);
    return handle_result(res);
}

SEXP first_complex__impl(SEXP x) {
    SEXP res = first_complex(x);
    return handle_result(res);
}

SEXP abs_complex__impl(SEXP x) {
    SEXP res = abs_complex(x);
    return handle_result(res);
}

SEXP new_value_pair__impl(SEXP a, SEXP b) {
    SEXP res = new_value_pair(a, b);
    return handle_result(res);
}

SEXP filter_integer_odd__impl(SEXP x) {
    SEXP res = filter_integer_odd(x);
    return handle_result(res);
}

SEXP filter_real_negative__impl(SEXP x) {
    SEXP res = filter_real_negative(x);
    return handle_result(res);
}

SEXP filter_complex_without_im__impl(SEXP x) {
    SEXP res = filter_complex_without_im(x);
    return handle_result(res);
}

SEXP filter_logical_duplicates__impl(SEXP x) {
    SEXP res = filter_logical_duplicates(x);
    return handle_result(res);
}

SEXP filter_string_ascii__impl(SEXP x) {
    SEXP res = filter_string_ascii(x);
    return handle_result(res);
}

SEXP print_foo_enum__impl(SEXP x) {
    SEXP res = print_foo_enum(x);
    return handle_result(res);
}

SEXP print_foo_enum_ref__impl(SEXP x) {
    SEXP res = print_foo_enum_ref(x);
    return handle_result(res);
}

SEXP foo_a__impl(void) {
    SEXP res = foo_a();
    return handle_result(res);
}

SEXP fun_mod1__impl(void) {
    SEXP res = fun_mod1();
    return handle_result(res);
}

SEXP fun_mod1_1_foo__impl(void) {
    SEXP res = fun_mod1_1_foo();
    return handle_result(res);
}

SEXP FooEnum_print__impl(SEXP self__) {
    SEXP res = FooEnum_print(self__);
    return handle_result(res);
}

SEXP Person_new__impl(void) {
    SEXP res = Person_new();
    return handle_result(res);
}

SEXP Person_new2__impl(void) {
    SEXP res = Person_new2();
    return handle_result(res);
}

SEXP Person_new_fallible__impl(void) {
    SEXP res = Person_new_fallible();
    return handle_result(res);
}

SEXP Person_another_person__impl(SEXP self__) {
    SEXP res = Person_another_person(self__);
    return handle_result(res);
}

SEXP Person_new_with_name__impl(SEXP name) {
    SEXP res = Person_new_with_name(name);
    return handle_result(res);
}

SEXP Person_set_name__impl(SEXP self__, SEXP name) {
    SEXP res = Person_set_name(self__, name);
    return handle_result(res);
}

SEXP Person_name__impl(SEXP self__) {
    SEXP res = Person_name(self__);
    return handle_result(res);
}

SEXP Person_associated_function__impl(void) {
    SEXP res = Person_associated_function();
    return handle_result(res);
}

SEXP Person2_name__impl(SEXP self__) {
    SEXP res = Person2_name(self__);
    return handle_result(res);
}

SEXP Value_new__impl(SEXP x) {
    SEXP res = Value_new(x);
    return handle_result(res);
}

SEXP Value_pair__impl(SEXP self__, SEXP b) {
    SEXP res = Value_pair(self__, b);
    return handle_result(res);
}

SEXP Value_get__impl(SEXP self__) {
    SEXP res = Value_get(self__);
    return handle_result(res);
}

SEXP Value_get2__impl(SEXP self__) {
    SEXP res = Value_get2(self__);
    return handle_result(res);
}

SEXP ValuePair_new__impl(SEXP a, SEXP b) {
    SEXP res = ValuePair_new(a, b);
    return handle_result(res);
}

SEXP ValuePair_new_copy__impl(SEXP a, SEXP b) {
    SEXP res = ValuePair_new_copy(a, b);
    return handle_result(res);
}

SEXP ValuePair_print__impl(SEXP self__) {
    SEXP res = ValuePair_print(self__);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"to_upper__impl", (DL_FUNC) &to_upper__impl, 1},
    {"add_suffix__impl", (DL_FUNC) &add_suffix__impl, 2},
    {"times_two_int__impl", (DL_FUNC) &times_two_int__impl, 1},
    {"times_any_int__impl", (DL_FUNC) &times_any_int__impl, 2},
    {"times_two_numeric__impl", (DL_FUNC) &times_two_numeric__impl, 1},
    {"times_any_numeric__impl", (DL_FUNC) &times_any_numeric__impl, 2},
    {"flip_logical__impl", (DL_FUNC) &flip_logical__impl, 1},
    {"flip_logical_expert_only__impl", (DL_FUNC) &flip_logical_expert_only__impl, 1},
    {"or_logical__impl", (DL_FUNC) &or_logical__impl, 2},
    {"print_list__impl", (DL_FUNC) &print_list__impl, 1},
    {"list_with_no_values__impl", (DL_FUNC) &list_with_no_values__impl, 0},
    {"list_with_no_names__impl", (DL_FUNC) &list_with_no_names__impl, 0},
    {"list_with_names_and_values__impl", (DL_FUNC) &list_with_names_and_values__impl, 0},
    {"external_person_new__impl", (DL_FUNC) &external_person_new__impl, 0},
    {"get_name_external__impl", (DL_FUNC) &get_name_external__impl, 1},
    {"set_name_external__impl", (DL_FUNC) &set_name_external__impl, 2},
    {"get_class_int__impl", (DL_FUNC) &get_class_int__impl, 1},
    {"get_names_int__impl", (DL_FUNC) &get_names_int__impl, 1},
    {"get_dim_int__impl", (DL_FUNC) &get_dim_int__impl, 1},
    {"get_attr_int__impl", (DL_FUNC) &get_attr_int__impl, 2},
    {"set_class_int__impl", (DL_FUNC) &set_class_int__impl, 0},
    {"set_names_int__impl", (DL_FUNC) &set_names_int__impl, 0},
    {"set_dim_int__impl", (DL_FUNC) &set_dim_int__impl, 0},
    {"set_attr_int__impl", (DL_FUNC) &set_attr_int__impl, 2},
    {"scalar_input_int__impl", (DL_FUNC) &scalar_input_int__impl, 1},
    {"scalar_input_usize__impl", (DL_FUNC) &scalar_input_usize__impl, 1},
    {"scalar_input_real__impl", (DL_FUNC) &scalar_input_real__impl, 1},
    {"scalar_input_logical__impl", (DL_FUNC) &scalar_input_logical__impl, 1},
    {"scalar_input_string__impl", (DL_FUNC) &scalar_input_string__impl, 1},
    {"scalar_output_int__impl", (DL_FUNC) &scalar_output_int__impl, 0},
    {"scalar_output_int2__impl", (DL_FUNC) &scalar_output_int2__impl, 0},
    {"scalar_output_real__impl", (DL_FUNC) &scalar_output_real__impl, 0},
    {"scalar_output_real2__impl", (DL_FUNC) &scalar_output_real2__impl, 0},
    {"scalar_output_logical__impl", (DL_FUNC) &scalar_output_logical__impl, 0},
    {"scalar_output_logical2__impl", (DL_FUNC) &scalar_output_logical2__impl, 0},
    {"scalar_output_string__impl", (DL_FUNC) &scalar_output_string__impl, 0},
    {"scalar_output_string2__impl", (DL_FUNC) &scalar_output_string2__impl, 0},
    {"scalar_output_complex__impl", (DL_FUNC) &scalar_output_complex__impl, 0},
    {"scalar_output_complex2__impl", (DL_FUNC) &scalar_output_complex2__impl, 0},
    {"sum_int__impl", (DL_FUNC) &sum_int__impl, 1},
    {"sum_real__impl", (DL_FUNC) &sum_real__impl, 1},
    {"rep_int_vec__impl", (DL_FUNC) &rep_int_vec__impl, 1},
    {"rep_int_slice__impl", (DL_FUNC) &rep_int_slice__impl, 1},
    {"rep_real_vec__impl", (DL_FUNC) &rep_real_vec__impl, 1},
    {"rep_real_slice__impl", (DL_FUNC) &rep_real_slice__impl, 1},
    {"rep_bool_vec__impl", (DL_FUNC) &rep_bool_vec__impl, 1},
    {"rep_bool_slice__impl", (DL_FUNC) &rep_bool_slice__impl, 1},
    {"rep_str_vec__impl", (DL_FUNC) &rep_str_vec__impl, 1},
    {"rep_str_slice__impl", (DL_FUNC) &rep_str_slice__impl, 1},
    {"safe_stop__impl", (DL_FUNC) &safe_stop__impl, 0},
    {"raise_error__impl", (DL_FUNC) &raise_error__impl, 0},
    {"new_int__impl", (DL_FUNC) &new_int__impl, 1},
    {"new_real__impl", (DL_FUNC) &new_real__impl, 1},
    {"new_bool__impl", (DL_FUNC) &new_bool__impl, 1},
    {"do_call__impl", (DL_FUNC) &do_call__impl, 2},
    {"call_with_args__impl", (DL_FUNC) &call_with_args__impl, 1},
    {"get_args__impl", (DL_FUNC) &get_args__impl, 1},
    {"new_complex__impl", (DL_FUNC) &new_complex__impl, 1},
    {"first_complex__impl", (DL_FUNC) &first_complex__impl, 1},
    {"abs_complex__impl", (DL_FUNC) &abs_complex__impl, 1},
    {"new_value_pair__impl", (DL_FUNC) &new_value_pair__impl, 2},
    {"filter_integer_odd__impl", (DL_FUNC) &filter_integer_odd__impl, 1},
    {"filter_real_negative__impl", (DL_FUNC) &filter_real_negative__impl, 1},
    {"filter_complex_without_im__impl", (DL_FUNC) &filter_complex_without_im__impl, 1},
    {"filter_logical_duplicates__impl", (DL_FUNC) &filter_logical_duplicates__impl, 1},
    {"filter_string_ascii__impl", (DL_FUNC) &filter_string_ascii__impl, 1},
    {"print_foo_enum__impl", (DL_FUNC) &print_foo_enum__impl, 1},
    {"print_foo_enum_ref__impl", (DL_FUNC) &print_foo_enum_ref__impl, 1},
    {"foo_a__impl", (DL_FUNC) &foo_a__impl, 0},
    {"fun_mod1__impl", (DL_FUNC) &fun_mod1__impl, 0},
    {"fun_mod1_1_foo__impl", (DL_FUNC) &fun_mod1_1_foo__impl, 0},
    {"FooEnum_print__impl", (DL_FUNC) &FooEnum_print__impl, 1},
    {"Person_new__impl", (DL_FUNC) &Person_new__impl, 0},
    {"Person_new2__impl", (DL_FUNC) &Person_new2__impl, 0},
    {"Person_new_fallible__impl", (DL_FUNC) &Person_new_fallible__impl, 0},
    {"Person_another_person__impl", (DL_FUNC) &Person_another_person__impl, 1},
    {"Person_new_with_name__impl", (DL_FUNC) &Person_new_with_name__impl, 1},
    {"Person_set_name__impl", (DL_FUNC) &Person_set_name__impl, 2},
    {"Person_name__impl", (DL_FUNC) &Person_name__impl, 1},
    {"Person_associated_function__impl", (DL_FUNC) &Person_associated_function__impl, 0},
    {"Person2_name__impl", (DL_FUNC) &Person2_name__impl, 1},
    {"Value_new__impl", (DL_FUNC) &Value_new__impl, 1},
    {"Value_pair__impl", (DL_FUNC) &Value_pair__impl, 2},
    {"Value_get__impl", (DL_FUNC) &Value_get__impl, 1},
    {"Value_get2__impl", (DL_FUNC) &Value_get2__impl, 1},
    {"ValuePair_new__impl", (DL_FUNC) &ValuePair_new__impl, 2},
    {"ValuePair_new_copy__impl", (DL_FUNC) &ValuePair_new_copy__impl, 2},
    {"ValuePair_print__impl", (DL_FUNC) &ValuePair_print__impl, 1},
    {NULL, NULL, 0}
};

void R_init_savvyExamples(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
