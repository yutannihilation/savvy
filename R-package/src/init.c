
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

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

SEXP savvy_is_built_with_debug__impl(void) {
    SEXP res = savvy_is_built_with_debug__ffi();
    return handle_result(res);
}

SEXP savvy_to_upper__impl(SEXP x) {
    SEXP res = savvy_to_upper__ffi(x);
    return handle_result(res);
}

SEXP savvy_add_suffix__impl(SEXP x, SEXP y) {
    SEXP res = savvy_add_suffix__ffi(x, y);
    return handle_result(res);
}

SEXP savvy_times_two_int__impl(SEXP x) {
    SEXP res = savvy_times_two_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_times_any_int__impl(SEXP x, SEXP y) {
    SEXP res = savvy_times_any_int__ffi(x, y);
    return handle_result(res);
}

SEXP savvy_times_two_numeric__impl(SEXP x) {
    SEXP res = savvy_times_two_numeric__ffi(x);
    return handle_result(res);
}

SEXP savvy_times_any_numeric__impl(SEXP x, SEXP y) {
    SEXP res = savvy_times_any_numeric__ffi(x, y);
    return handle_result(res);
}

SEXP savvy_flip_logical__impl(SEXP x) {
    SEXP res = savvy_flip_logical__ffi(x);
    return handle_result(res);
}

SEXP savvy_flip_logical_expert_only__impl(SEXP x) {
    SEXP res = savvy_flip_logical_expert_only__ffi(x);
    return handle_result(res);
}

SEXP savvy_or_logical__impl(SEXP x, SEXP y) {
    SEXP res = savvy_or_logical__ffi(x, y);
    return handle_result(res);
}

SEXP savvy_print_list__impl(SEXP x) {
    SEXP res = savvy_print_list__ffi(x);
    return handle_result(res);
}

SEXP savvy_list_with_no_values__impl(void) {
    SEXP res = savvy_list_with_no_values__ffi();
    return handle_result(res);
}

SEXP savvy_list_with_no_names__impl(void) {
    SEXP res = savvy_list_with_no_names__ffi();
    return handle_result(res);
}

SEXP savvy_list_with_names_and_values__impl(void) {
    SEXP res = savvy_list_with_names_and_values__ffi();
    return handle_result(res);
}

SEXP savvy_external_person_new__impl(void) {
    SEXP res = savvy_external_person_new__ffi();
    return handle_result(res);
}

SEXP savvy_get_name_external__impl(SEXP x) {
    SEXP res = savvy_get_name_external__ffi(x);
    return handle_result(res);
}

SEXP savvy_set_name_external__impl(SEXP x, SEXP name) {
    SEXP res = savvy_set_name_external__ffi(x, name);
    return handle_result(res);
}

SEXP savvy_altint__impl(void) {
    SEXP res = savvy_altint__ffi();
    return handle_result(res);
}

SEXP savvy_altint_mutable__impl(void) {
    SEXP res = savvy_altint_mutable__ffi();
    return handle_result(res);
}

SEXP savvy_print_altint__impl(SEXP x) {
    SEXP res = savvy_print_altint__ffi(x);
    return handle_result(res);
}

SEXP savvy_tweak_altint__impl(SEXP x) {
    SEXP res = savvy_tweak_altint__ffi(x);
    return handle_result(res);
}

SEXP savvy_altreal__impl(void) {
    SEXP res = savvy_altreal__ffi();
    return handle_result(res);
}

SEXP savvy_altreal_mutable__impl(void) {
    SEXP res = savvy_altreal_mutable__ffi();
    return handle_result(res);
}

SEXP savvy_print_altreal__impl(SEXP x) {
    SEXP res = savvy_print_altreal__ffi(x);
    return handle_result(res);
}

SEXP savvy_tweak_altreal__impl(SEXP x) {
    SEXP res = savvy_tweak_altreal__ffi(x);
    return handle_result(res);
}

SEXP savvy_altlogical__impl(void) {
    SEXP res = savvy_altlogical__ffi();
    return handle_result(res);
}

SEXP savvy_altlogical_mutable__impl(void) {
    SEXP res = savvy_altlogical_mutable__ffi();
    return handle_result(res);
}

SEXP savvy_print_altlogical__impl(SEXP x) {
    SEXP res = savvy_print_altlogical__ffi(x);
    return handle_result(res);
}

SEXP savvy_tweak_altlogical__impl(SEXP x) {
    SEXP res = savvy_tweak_altlogical__ffi(x);
    return handle_result(res);
}

SEXP savvy_altstring__impl(void) {
    SEXP res = savvy_altstring__ffi();
    return handle_result(res);
}

SEXP savvy_altstring_mutable__impl(void) {
    SEXP res = savvy_altstring_mutable__ffi();
    return handle_result(res);
}

SEXP savvy_print_altstring__impl(SEXP x) {
    SEXP res = savvy_print_altstring__ffi(x);
    return handle_result(res);
}

SEXP savvy_tweak_altstring__impl(SEXP x) {
    SEXP res = savvy_tweak_altstring__ffi(x);
    return handle_result(res);
}

SEXP savvy_init_altrep_class__impl(DllInfo* dll_info) {
    SEXP res = savvy_init_altrep_class__ffi(dll_info);
    return handle_result(res);
}

SEXP savvy_get_class_int__impl(SEXP x) {
    SEXP res = savvy_get_class_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_get_names_int__impl(SEXP x) {
    SEXP res = savvy_get_names_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_get_dim_int__impl(SEXP x) {
    SEXP res = savvy_get_dim_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_get_attr_int__impl(SEXP x, SEXP attr) {
    SEXP res = savvy_get_attr_int__ffi(x, attr);
    return handle_result(res);
}

SEXP savvy_set_class_int__impl(void) {
    SEXP res = savvy_set_class_int__ffi();
    return handle_result(res);
}

SEXP savvy_set_names_int__impl(void) {
    SEXP res = savvy_set_names_int__ffi();
    return handle_result(res);
}

SEXP savvy_set_dim_int__impl(void) {
    SEXP res = savvy_set_dim_int__ffi();
    return handle_result(res);
}

SEXP savvy_set_attr_int__impl(SEXP attr, SEXP value) {
    SEXP res = savvy_set_attr_int__ffi(attr, value);
    return handle_result(res);
}

SEXP savvy_new_complex__impl(SEXP size) {
    SEXP res = savvy_new_complex__ffi(size);
    return handle_result(res);
}

SEXP savvy_first_complex__impl(SEXP x) {
    SEXP res = savvy_first_complex__ffi(x);
    return handle_result(res);
}

SEXP savvy_abs_complex__impl(SEXP x) {
    SEXP res = savvy_abs_complex__ffi(x);
    return handle_result(res);
}

SEXP savvy_new_value_pair__impl(SEXP a, SEXP b) {
    SEXP res = savvy_new_value_pair__ffi(a, b);
    return handle_result(res);
}

SEXP savvy_scalar_input_int__impl(SEXP x) {
    SEXP res = savvy_scalar_input_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_usize__impl(SEXP x) {
    SEXP res = savvy_scalar_input_usize__ffi(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_real__impl(SEXP x) {
    SEXP res = savvy_scalar_input_real__ffi(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_logical__impl(SEXP x) {
    SEXP res = savvy_scalar_input_logical__ffi(x);
    return handle_result(res);
}

SEXP savvy_scalar_input_string__impl(SEXP x) {
    SEXP res = savvy_scalar_input_string__ffi(x);
    return handle_result(res);
}

SEXP savvy_scalar_output_int__impl(void) {
    SEXP res = savvy_scalar_output_int__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_int2__impl(void) {
    SEXP res = savvy_scalar_output_int2__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_real__impl(void) {
    SEXP res = savvy_scalar_output_real__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_real2__impl(void) {
    SEXP res = savvy_scalar_output_real2__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_logical__impl(void) {
    SEXP res = savvy_scalar_output_logical__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_logical2__impl(void) {
    SEXP res = savvy_scalar_output_logical2__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_string__impl(void) {
    SEXP res = savvy_scalar_output_string__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_string2__impl(void) {
    SEXP res = savvy_scalar_output_string2__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_complex__impl(void) {
    SEXP res = savvy_scalar_output_complex__ffi();
    return handle_result(res);
}

SEXP savvy_scalar_output_complex2__impl(void) {
    SEXP res = savvy_scalar_output_complex2__ffi();
    return handle_result(res);
}

SEXP savvy_sum_int__impl(SEXP x) {
    SEXP res = savvy_sum_int__ffi(x);
    return handle_result(res);
}

SEXP savvy_sum_real__impl(SEXP x) {
    SEXP res = savvy_sum_real__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_int_vec__impl(SEXP x) {
    SEXP res = savvy_rep_int_vec__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_int_slice__impl(SEXP x) {
    SEXP res = savvy_rep_int_slice__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_real_vec__impl(SEXP x) {
    SEXP res = savvy_rep_real_vec__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_real_slice__impl(SEXP x) {
    SEXP res = savvy_rep_real_slice__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_bool_vec__impl(SEXP x) {
    SEXP res = savvy_rep_bool_vec__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_bool_slice__impl(SEXP x) {
    SEXP res = savvy_rep_bool_slice__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_str_vec__impl(SEXP x) {
    SEXP res = savvy_rep_str_vec__ffi(x);
    return handle_result(res);
}

SEXP savvy_rep_str_slice__impl(SEXP x) {
    SEXP res = savvy_rep_str_slice__ffi(x);
    return handle_result(res);
}

SEXP savvy_print_foo_enum__impl(SEXP x) {
    SEXP res = savvy_print_foo_enum__ffi(x);
    return handle_result(res);
}

SEXP savvy_print_foo_enum_ref__impl(SEXP x) {
    SEXP res = savvy_print_foo_enum_ref__ffi(x);
    return handle_result(res);
}

SEXP savvy_foo_a__impl(void) {
    SEXP res = savvy_foo_a__ffi();
    return handle_result(res);
}

SEXP savvy_safe_stop__impl(void) {
    SEXP res = savvy_safe_stop__ffi();
    return handle_result(res);
}

SEXP savvy_raise_error__impl(void) {
    SEXP res = savvy_raise_error__ffi();
    return handle_result(res);
}

SEXP savvy_must_panic__impl(void) {
    SEXP res = savvy_must_panic__ffi();
    return handle_result(res);
}

SEXP savvy_do_call__impl(SEXP fun, SEXP args) {
    SEXP res = savvy_do_call__ffi(fun, args);
    return handle_result(res);
}

SEXP savvy_call_with_args__impl(SEXP fun) {
    SEXP res = savvy_call_with_args__ffi(fun);
    return handle_result(res);
}

SEXP savvy_get_args__impl(SEXP args) {
    SEXP res = savvy_get_args__ffi(args);
    return handle_result(res);
}

SEXP savvy_new_int__impl(SEXP size) {
    SEXP res = savvy_new_int__ffi(size);
    return handle_result(res);
}

SEXP savvy_new_real__impl(SEXP size) {
    SEXP res = savvy_new_real__ffi(size);
    return handle_result(res);
}

SEXP savvy_new_bool__impl(SEXP size) {
    SEXP res = savvy_new_bool__ffi(size);
    return handle_result(res);
}

SEXP savvy_filter_integer_odd__impl(SEXP x) {
    SEXP res = savvy_filter_integer_odd__ffi(x);
    return handle_result(res);
}

SEXP savvy_filter_real_negative__impl(SEXP x) {
    SEXP res = savvy_filter_real_negative__ffi(x);
    return handle_result(res);
}

SEXP savvy_filter_complex_without_im__impl(SEXP x) {
    SEXP res = savvy_filter_complex_without_im__ffi(x);
    return handle_result(res);
}

SEXP savvy_filter_logical_duplicates__impl(SEXP x) {
    SEXP res = savvy_filter_logical_duplicates__ffi(x);
    return handle_result(res);
}

SEXP savvy_filter_string_ascii__impl(SEXP x) {
    SEXP res = savvy_filter_string_ascii__ffi(x);
    return handle_result(res);
}

SEXP savvy_fun_mod1__impl(void) {
    SEXP res = savvy_fun_mod1__ffi();
    return handle_result(res);
}

SEXP savvy_fun_mod1_1_foo__impl(void) {
    SEXP res = savvy_fun_mod1_1_foo__ffi();
    return handle_result(res);
}

SEXP savvy_FooEnum_print__impl(SEXP self__) {
    SEXP res = savvy_FooEnum_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Person_new__impl(void) {
    SEXP res = savvy_Person_new__ffi();
    return handle_result(res);
}

SEXP savvy_Person_new2__impl(void) {
    SEXP res = savvy_Person_new2__ffi();
    return handle_result(res);
}

SEXP savvy_Person_new_fallible__impl(void) {
    SEXP res = savvy_Person_new_fallible__ffi();
    return handle_result(res);
}

SEXP savvy_Person_another_person__impl(SEXP self__) {
    SEXP res = savvy_Person_another_person__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Person_new_with_name__impl(SEXP name) {
    SEXP res = savvy_Person_new_with_name__ffi(name);
    return handle_result(res);
}

SEXP savvy_Person_set_name__impl(SEXP self__, SEXP name) {
    SEXP res = savvy_Person_set_name__ffi(self__, name);
    return handle_result(res);
}

SEXP savvy_Person_name__impl(SEXP self__) {
    SEXP res = savvy_Person_name__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Person_associated_function__impl(void) {
    SEXP res = savvy_Person_associated_function__ffi();
    return handle_result(res);
}

SEXP savvy_Person2_name__impl(SEXP self__) {
    SEXP res = savvy_Person2_name__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Value_new__impl(SEXP x) {
    SEXP res = savvy_Value_new__ffi(x);
    return handle_result(res);
}

SEXP savvy_Value_pair__impl(SEXP self__, SEXP b) {
    SEXP res = savvy_Value_pair__ffi(self__, b);
    return handle_result(res);
}

SEXP savvy_Value_get__impl(SEXP self__) {
    SEXP res = savvy_Value_get__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Value_get2__impl(SEXP self__) {
    SEXP res = savvy_Value_get2__ffi(self__);
    return handle_result(res);
}

SEXP savvy_ValuePair_new__impl(SEXP a, SEXP b) {
    SEXP res = savvy_ValuePair_new__ffi(a, b);
    return handle_result(res);
}

SEXP savvy_ValuePair_new_copy__impl(SEXP a, SEXP b) {
    SEXP res = savvy_ValuePair_new_copy__ffi(a, b);
    return handle_result(res);
}

SEXP savvy_ValuePair_print__impl(SEXP self__) {
    SEXP res = savvy_ValuePair_print__ffi(self__);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_is_built_with_debug__impl", (DL_FUNC) &savvy_is_built_with_debug__impl, 0},
    {"savvy_to_upper__impl", (DL_FUNC) &savvy_to_upper__impl, 1},
    {"savvy_add_suffix__impl", (DL_FUNC) &savvy_add_suffix__impl, 2},
    {"savvy_times_two_int__impl", (DL_FUNC) &savvy_times_two_int__impl, 1},
    {"savvy_times_any_int__impl", (DL_FUNC) &savvy_times_any_int__impl, 2},
    {"savvy_times_two_numeric__impl", (DL_FUNC) &savvy_times_two_numeric__impl, 1},
    {"savvy_times_any_numeric__impl", (DL_FUNC) &savvy_times_any_numeric__impl, 2},
    {"savvy_flip_logical__impl", (DL_FUNC) &savvy_flip_logical__impl, 1},
    {"savvy_flip_logical_expert_only__impl", (DL_FUNC) &savvy_flip_logical_expert_only__impl, 1},
    {"savvy_or_logical__impl", (DL_FUNC) &savvy_or_logical__impl, 2},
    {"savvy_print_list__impl", (DL_FUNC) &savvy_print_list__impl, 1},
    {"savvy_list_with_no_values__impl", (DL_FUNC) &savvy_list_with_no_values__impl, 0},
    {"savvy_list_with_no_names__impl", (DL_FUNC) &savvy_list_with_no_names__impl, 0},
    {"savvy_list_with_names_and_values__impl", (DL_FUNC) &savvy_list_with_names_and_values__impl, 0},
    {"savvy_external_person_new__impl", (DL_FUNC) &savvy_external_person_new__impl, 0},
    {"savvy_get_name_external__impl", (DL_FUNC) &savvy_get_name_external__impl, 1},
    {"savvy_set_name_external__impl", (DL_FUNC) &savvy_set_name_external__impl, 2},
    {"savvy_altint__impl", (DL_FUNC) &savvy_altint__impl, 0},
    {"savvy_altint_mutable__impl", (DL_FUNC) &savvy_altint_mutable__impl, 0},
    {"savvy_print_altint__impl", (DL_FUNC) &savvy_print_altint__impl, 1},
    {"savvy_tweak_altint__impl", (DL_FUNC) &savvy_tweak_altint__impl, 1},
    {"savvy_altreal__impl", (DL_FUNC) &savvy_altreal__impl, 0},
    {"savvy_altreal_mutable__impl", (DL_FUNC) &savvy_altreal_mutable__impl, 0},
    {"savvy_print_altreal__impl", (DL_FUNC) &savvy_print_altreal__impl, 1},
    {"savvy_tweak_altreal__impl", (DL_FUNC) &savvy_tweak_altreal__impl, 1},
    {"savvy_altlogical__impl", (DL_FUNC) &savvy_altlogical__impl, 0},
    {"savvy_altlogical_mutable__impl", (DL_FUNC) &savvy_altlogical_mutable__impl, 0},
    {"savvy_print_altlogical__impl", (DL_FUNC) &savvy_print_altlogical__impl, 1},
    {"savvy_tweak_altlogical__impl", (DL_FUNC) &savvy_tweak_altlogical__impl, 1},
    {"savvy_altstring__impl", (DL_FUNC) &savvy_altstring__impl, 0},
    {"savvy_altstring_mutable__impl", (DL_FUNC) &savvy_altstring_mutable__impl, 0},
    {"savvy_print_altstring__impl", (DL_FUNC) &savvy_print_altstring__impl, 1},
    {"savvy_tweak_altstring__impl", (DL_FUNC) &savvy_tweak_altstring__impl, 1},
    {"savvy_get_class_int__impl", (DL_FUNC) &savvy_get_class_int__impl, 1},
    {"savvy_get_names_int__impl", (DL_FUNC) &savvy_get_names_int__impl, 1},
    {"savvy_get_dim_int__impl", (DL_FUNC) &savvy_get_dim_int__impl, 1},
    {"savvy_get_attr_int__impl", (DL_FUNC) &savvy_get_attr_int__impl, 2},
    {"savvy_set_class_int__impl", (DL_FUNC) &savvy_set_class_int__impl, 0},
    {"savvy_set_names_int__impl", (DL_FUNC) &savvy_set_names_int__impl, 0},
    {"savvy_set_dim_int__impl", (DL_FUNC) &savvy_set_dim_int__impl, 0},
    {"savvy_set_attr_int__impl", (DL_FUNC) &savvy_set_attr_int__impl, 2},
    {"savvy_new_complex__impl", (DL_FUNC) &savvy_new_complex__impl, 1},
    {"savvy_first_complex__impl", (DL_FUNC) &savvy_first_complex__impl, 1},
    {"savvy_abs_complex__impl", (DL_FUNC) &savvy_abs_complex__impl, 1},
    {"savvy_new_value_pair__impl", (DL_FUNC) &savvy_new_value_pair__impl, 2},
    {"savvy_scalar_input_int__impl", (DL_FUNC) &savvy_scalar_input_int__impl, 1},
    {"savvy_scalar_input_usize__impl", (DL_FUNC) &savvy_scalar_input_usize__impl, 1},
    {"savvy_scalar_input_real__impl", (DL_FUNC) &savvy_scalar_input_real__impl, 1},
    {"savvy_scalar_input_logical__impl", (DL_FUNC) &savvy_scalar_input_logical__impl, 1},
    {"savvy_scalar_input_string__impl", (DL_FUNC) &savvy_scalar_input_string__impl, 1},
    {"savvy_scalar_output_int__impl", (DL_FUNC) &savvy_scalar_output_int__impl, 0},
    {"savvy_scalar_output_int2__impl", (DL_FUNC) &savvy_scalar_output_int2__impl, 0},
    {"savvy_scalar_output_real__impl", (DL_FUNC) &savvy_scalar_output_real__impl, 0},
    {"savvy_scalar_output_real2__impl", (DL_FUNC) &savvy_scalar_output_real2__impl, 0},
    {"savvy_scalar_output_logical__impl", (DL_FUNC) &savvy_scalar_output_logical__impl, 0},
    {"savvy_scalar_output_logical2__impl", (DL_FUNC) &savvy_scalar_output_logical2__impl, 0},
    {"savvy_scalar_output_string__impl", (DL_FUNC) &savvy_scalar_output_string__impl, 0},
    {"savvy_scalar_output_string2__impl", (DL_FUNC) &savvy_scalar_output_string2__impl, 0},
    {"savvy_scalar_output_complex__impl", (DL_FUNC) &savvy_scalar_output_complex__impl, 0},
    {"savvy_scalar_output_complex2__impl", (DL_FUNC) &savvy_scalar_output_complex2__impl, 0},
    {"savvy_sum_int__impl", (DL_FUNC) &savvy_sum_int__impl, 1},
    {"savvy_sum_real__impl", (DL_FUNC) &savvy_sum_real__impl, 1},
    {"savvy_rep_int_vec__impl", (DL_FUNC) &savvy_rep_int_vec__impl, 1},
    {"savvy_rep_int_slice__impl", (DL_FUNC) &savvy_rep_int_slice__impl, 1},
    {"savvy_rep_real_vec__impl", (DL_FUNC) &savvy_rep_real_vec__impl, 1},
    {"savvy_rep_real_slice__impl", (DL_FUNC) &savvy_rep_real_slice__impl, 1},
    {"savvy_rep_bool_vec__impl", (DL_FUNC) &savvy_rep_bool_vec__impl, 1},
    {"savvy_rep_bool_slice__impl", (DL_FUNC) &savvy_rep_bool_slice__impl, 1},
    {"savvy_rep_str_vec__impl", (DL_FUNC) &savvy_rep_str_vec__impl, 1},
    {"savvy_rep_str_slice__impl", (DL_FUNC) &savvy_rep_str_slice__impl, 1},
    {"savvy_print_foo_enum__impl", (DL_FUNC) &savvy_print_foo_enum__impl, 1},
    {"savvy_print_foo_enum_ref__impl", (DL_FUNC) &savvy_print_foo_enum_ref__impl, 1},
    {"savvy_foo_a__impl", (DL_FUNC) &savvy_foo_a__impl, 0},
    {"savvy_safe_stop__impl", (DL_FUNC) &savvy_safe_stop__impl, 0},
    {"savvy_raise_error__impl", (DL_FUNC) &savvy_raise_error__impl, 0},
    {"savvy_must_panic__impl", (DL_FUNC) &savvy_must_panic__impl, 0},
    {"savvy_do_call__impl", (DL_FUNC) &savvy_do_call__impl, 2},
    {"savvy_call_with_args__impl", (DL_FUNC) &savvy_call_with_args__impl, 1},
    {"savvy_get_args__impl", (DL_FUNC) &savvy_get_args__impl, 1},
    {"savvy_new_int__impl", (DL_FUNC) &savvy_new_int__impl, 1},
    {"savvy_new_real__impl", (DL_FUNC) &savvy_new_real__impl, 1},
    {"savvy_new_bool__impl", (DL_FUNC) &savvy_new_bool__impl, 1},
    {"savvy_filter_integer_odd__impl", (DL_FUNC) &savvy_filter_integer_odd__impl, 1},
    {"savvy_filter_real_negative__impl", (DL_FUNC) &savvy_filter_real_negative__impl, 1},
    {"savvy_filter_complex_without_im__impl", (DL_FUNC) &savvy_filter_complex_without_im__impl, 1},
    {"savvy_filter_logical_duplicates__impl", (DL_FUNC) &savvy_filter_logical_duplicates__impl, 1},
    {"savvy_filter_string_ascii__impl", (DL_FUNC) &savvy_filter_string_ascii__impl, 1},
    {"savvy_fun_mod1__impl", (DL_FUNC) &savvy_fun_mod1__impl, 0},
    {"savvy_fun_mod1_1_foo__impl", (DL_FUNC) &savvy_fun_mod1_1_foo__impl, 0},
    {"savvy_FooEnum_print__impl", (DL_FUNC) &savvy_FooEnum_print__impl, 1},
    {"savvy_Person_new__impl", (DL_FUNC) &savvy_Person_new__impl, 0},
    {"savvy_Person_new2__impl", (DL_FUNC) &savvy_Person_new2__impl, 0},
    {"savvy_Person_new_fallible__impl", (DL_FUNC) &savvy_Person_new_fallible__impl, 0},
    {"savvy_Person_another_person__impl", (DL_FUNC) &savvy_Person_another_person__impl, 1},
    {"savvy_Person_new_with_name__impl", (DL_FUNC) &savvy_Person_new_with_name__impl, 1},
    {"savvy_Person_set_name__impl", (DL_FUNC) &savvy_Person_set_name__impl, 2},
    {"savvy_Person_name__impl", (DL_FUNC) &savvy_Person_name__impl, 1},
    {"savvy_Person_associated_function__impl", (DL_FUNC) &savvy_Person_associated_function__impl, 0},
    {"savvy_Person2_name__impl", (DL_FUNC) &savvy_Person2_name__impl, 1},
    {"savvy_Value_new__impl", (DL_FUNC) &savvy_Value_new__impl, 1},
    {"savvy_Value_pair__impl", (DL_FUNC) &savvy_Value_pair__impl, 2},
    {"savvy_Value_get__impl", (DL_FUNC) &savvy_Value_get__impl, 1},
    {"savvy_Value_get2__impl", (DL_FUNC) &savvy_Value_get2__impl, 1},
    {"savvy_ValuePair_new__impl", (DL_FUNC) &savvy_ValuePair_new__impl, 2},
    {"savvy_ValuePair_new_copy__impl", (DL_FUNC) &savvy_ValuePair_new_copy__impl, 2},
    {"savvy_ValuePair_print__impl", (DL_FUNC) &savvy_ValuePair_print__impl, 1},
    {NULL, NULL, 0}
};

void R_init_savvyExamples(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.
    savvy_init_altrep_class__impl(dll);
}
