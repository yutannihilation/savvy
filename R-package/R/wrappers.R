#' @useDynLib savvyExamples, .registration = TRUE
#' @keywords internal
NULL

# Check class and extract the external pointer embedded in the environment
.savvy_extract_ptr <- function(e, class) {
  if(inherits(e, class)) {
    e$.ptr
  } else {
    msg <- paste0("Expected ", class, ", got ", class(e)[1])
    stop(msg, call. = FALSE)
  }
}

#' Convert Input To Upper-Case
#'
#' @param x A character vector.
#' @returns A character vector with upper case version of the input.
#' @export
to_upper <- function(x) {
  .Call(to_upper__impl, x)
}

#' Add suffix
#'
#' @param x A character vector.
#' @param y A suffix.
#' @returns A character vector with upper case version of the input.
#' @export
add_suffix <- function(x, y) {
  .Call(add_suffix__impl, x, y)
}

#' Multiply Input By Two
#'
#' @param x An integer vector.
#' @returns An integer vector with values multiplied by 2.
#' @export
times_two_int <- function(x) {
  .Call(times_two_int__impl, x)
}

#' Multiply Input By Another Input
#'
#' @param x An integer vector.
#' @param y An integer to multiply.
#' @returns An integer vector with values multiplied by `y`.
#' @export
times_any_int <- function(x, y) {
  .Call(times_any_int__impl, x, y)
}

#' Multiply Input By Two
#'
#' @param x A numeric vector.
#' @returns A numeric vector with values multiplied by 2.
#' @export
times_two_numeric <- function(x) {
  .Call(times_two_numeric__impl, x)
}

#' Multiply Input By Another Input
#'
#' @param x A real vector.
#' @param y A real to multiply.
#' @returns A real vector with values multiplied by `y`.
#' @export
times_any_numeric <- function(x, y) {
  .Call(times_any_numeric__impl, x, y)
}

#' Flip Input
#'
#' @param x A logical vector.
#' @returns A logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
flip_logical <- function(x) {
  .Call(flip_logical__impl, x)
}


flip_logical_expert_only <- function(x) {
  .Call(flip_logical_expert_only__impl, x)
}

#' Or operation
#'
#' @param x A logical vector.
#' @param y A logical value.
#' @returns A logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
or_logical <- function(x, y) {
  .Call(or_logical__impl, x, y)
}

#' Print the content of list
#'
#' @param x A list vector.
#' @returns `NULL`
#' @export
print_list <- function(x) {
  invisible(.Call(print_list__impl, x))
}


list_with_no_values <- function() {
  .Call(list_with_no_values__impl)
}


list_with_no_names <- function() {
  .Call(list_with_no_names__impl)
}


list_with_names_and_values <- function() {
  .Call(list_with_names_and_values__impl)
}


external_person_new <- function() {
  .savvy_wrap_Person(.Call(external_person_new__impl))
}


get_name_external <- function(x) {
  x <- .savvy_extract_ptr(x, "Person")
  .Call(get_name_external__impl, x)
}


set_name_external <- function(x, name) {
  x <- .savvy_extract_ptr(x, "Person")
  invisible(.Call(set_name_external__impl, x, name))
}


get_class_int <- function(x) {
  .Call(get_class_int__impl, x)
}


get_names_int <- function(x) {
  .Call(get_names_int__impl, x)
}


get_dim_int <- function(x) {
  .Call(get_dim_int__impl, x)
}


get_attr_int <- function(x, attr) {
  .Call(get_attr_int__impl, x, attr)
}


set_class_int <- function() {
  .Call(set_class_int__impl)
}


set_names_int <- function() {
  .Call(set_names_int__impl)
}


set_dim_int <- function() {
  .Call(set_dim_int__impl)
}


set_attr_int <- function(attr, value) {
  .Call(set_attr_int__impl, attr, value)
}


scalar_input_int <- function(x) {
  invisible(.Call(scalar_input_int__impl, x))
}


scalar_input_usize <- function(x) {
  invisible(.Call(scalar_input_usize__impl, x))
}


scalar_input_real <- function(x) {
  invisible(.Call(scalar_input_real__impl, x))
}


scalar_input_logical <- function(x) {
  invisible(.Call(scalar_input_logical__impl, x))
}


scalar_input_string <- function(x) {
  invisible(.Call(scalar_input_string__impl, x))
}


scalar_output_int <- function() {
  .Call(scalar_output_int__impl)
}


scalar_output_int2 <- function() {
  .Call(scalar_output_int2__impl)
}


scalar_output_real <- function() {
  .Call(scalar_output_real__impl)
}


scalar_output_real2 <- function() {
  .Call(scalar_output_real2__impl)
}


scalar_output_logical <- function() {
  .Call(scalar_output_logical__impl)
}


scalar_output_logical2 <- function() {
  .Call(scalar_output_logical2__impl)
}


scalar_output_string <- function() {
  .Call(scalar_output_string__impl)
}


scalar_output_string2 <- function() {
  .Call(scalar_output_string2__impl)
}


scalar_output_complex <- function() {
  .Call(scalar_output_complex__impl)
}


scalar_output_complex2 <- function() {
  .Call(scalar_output_complex2__impl)
}


sum_int <- function(x) {
  .Call(sum_int__impl, x)
}


sum_real <- function(x) {
  .Call(sum_real__impl, x)
}


rep_int_vec <- function(x) {
  .Call(rep_int_vec__impl, x)
}


rep_int_slice <- function(x) {
  .Call(rep_int_slice__impl, x)
}


rep_real_vec <- function(x) {
  .Call(rep_real_vec__impl, x)
}


rep_real_slice <- function(x) {
  .Call(rep_real_slice__impl, x)
}


rep_bool_vec <- function(x) {
  .Call(rep_bool_vec__impl, x)
}


rep_bool_slice <- function(x) {
  .Call(rep_bool_slice__impl, x)
}


rep_str_vec <- function(x) {
  .Call(rep_str_vec__impl, x)
}


rep_str_slice <- function(x) {
  .Call(rep_str_slice__impl, x)
}


safe_stop <- function() {
  invisible(.Call(safe_stop__impl))
}


raise_error <- function() {
  .Call(raise_error__impl)
}


new_int <- function(size) {
  .Call(new_int__impl, size)
}


new_real <- function(size) {
  .Call(new_real__impl, size)
}


new_bool <- function(size) {
  .Call(new_bool__impl, size)
}


do_call <- function(fun, args) {
  .Call(do_call__impl, fun, args)
}


call_with_args <- function(fun) {
  .Call(call_with_args__impl, fun)
}


get_args <- function(args) {
  .Call(get_args__impl, args)
}


new_complex <- function(size) {
  .Call(new_complex__impl, size)
}


first_complex <- function(x) {
  .Call(first_complex__impl, x)
}


abs_complex <- function(x) {
  .Call(abs_complex__impl, x)
}


new_value_pair <- function(a, b) {
  a <- .savvy_extract_ptr(a, "Value")
  b <- .savvy_extract_ptr(b, "Value")
  .savvy_wrap_ValuePair(.Call(new_value_pair__impl, a, b))
}


filter_integer_odd <- function(x) {
  .Call(filter_integer_odd__impl, x)
}


filter_real_negative <- function(x) {
  .Call(filter_real_negative__impl, x)
}


filter_complex_without_im <- function(x) {
  .Call(filter_complex_without_im__impl, x)
}


filter_logical_duplicates <- function(x) {
  .Call(filter_logical_duplicates__impl, x)
}


filter_string_ascii <- function(x) {
  .Call(filter_string_ascii__impl, x)
}


fun_mod1 <- function() {
  invisible(.Call(fun_mod1__impl))
}


fun_mod1_1_foo <- function() {
  invisible(.Call(fun_mod1_1_foo__impl))
}

#' A person with a name
#'
#' @export
Person <- new.env(parent = emptyenv())
Person$new <- function() {
  .savvy_wrap_Person(.Call(Person_new__impl))
}

Person$new2 <- function() {
  .savvy_wrap_Person(.Call(Person_new2__impl))
}

Person$new_fallible <- function() {
  .savvy_wrap_Person(.Call(Person_new_fallible__impl))
}

Person$new_with_name <- function(name) {
  .savvy_wrap_Person(.Call(Person_new_with_name__impl, name))
}

Person$associated_function <- function() {
.Call(Person_associated_function__impl)
}


.savvy_wrap_Person <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$another_person <- Person_another_person(ptr)
  e$set_name <- Person_set_name(ptr)
  e$name <- Person_name(ptr)

  class(e) <- "Person"
  e
}


Person_another_person <- function(self) {
  function() {
    .savvy_wrap_Person2(.Call(Person_another_person__impl, self))
  }
}

Person_set_name <- function(self) {
  function(name) {
  invisible(.Call(Person_set_name__impl, self, name))
  }
}

Person_name <- function(self) {
  function() {
  .Call(Person_name__impl, self)
  }
}



Person2 <- new.env(parent = emptyenv())


.savvy_wrap_Person2 <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$name <- Person2_name(ptr)

  class(e) <- "Person2"
  e
}


Person2_name <- function(self) {
  function() {
  .Call(Person2_name__impl, self)
  }
}



Value <- new.env(parent = emptyenv())
Value$new <- function(x) {
  .savvy_wrap_Value(.Call(Value_new__impl, x))
}


.savvy_wrap_Value <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$pair <- Value_pair(ptr)
  e$get <- Value_get(ptr)
  e$get2 <- Value_get2(ptr)

  class(e) <- "Value"
  e
}


Value_pair <- function(self) {
  function(b) {
    b <- .savvy_extract_ptr(b, "Value")
  .savvy_wrap_ValuePair(.Call(Value_pair__impl, self, b))
  }
}

Value_get <- function(self) {
  function() {
  .Call(Value_get__impl, self)
  }
}

Value_get2 <- function(self) {
  function() {
  .Call(Value_get2__impl, self)
  }
}



ValuePair <- new.env(parent = emptyenv())
ValuePair$new <- function(a, b) {
  a <- .savvy_extract_ptr(a, "Value")
  b <- .savvy_extract_ptr(b, "Value")
  .savvy_wrap_ValuePair(.Call(ValuePair_new__impl, a, b))
}

ValuePair$new_copy <- function(a, b) {
  a <- .savvy_extract_ptr(a, "Value")
  b <- .savvy_extract_ptr(b, "Value")
  .savvy_wrap_ValuePair(.Call(ValuePair_new_copy__impl, a, b))
}


.savvy_wrap_ValuePair <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- ValuePair_print(ptr)

  class(e) <- "ValuePair"
  e
}


ValuePair_print <- function(self) {
  function() {
  invisible(.Call(ValuePair_print__impl, self))
  }
}


#' @export
Foo <- list(
  A = 0L,
  B = 1L
)
