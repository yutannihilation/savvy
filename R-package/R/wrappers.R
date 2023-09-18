#' @useDynLib savvy, .registration = TRUE
#' @keywords internal
"_PACKAGE"


scalar_input_int <- function(x) {
  invisible(.Call(savvy_scalar_input_int, x))
}


scalar_input_real <- function(x) {
  invisible(.Call(savvy_scalar_input_real, x))
}


scalar_input_logical <- function(x) {
  invisible(.Call(savvy_scalar_input_logical, x))
}


scalar_input_str <- function(x) {
  invisible(.Call(savvy_scalar_input_str, x))
}

#' Convert Input To Upper-Case
#'
#' @param x A character vector.
#' @returns A character vector with upper case version of the input.
#' @export
to_upper <- function(x) {
  .Call(savvy_to_upper, x)
}

#' Add suffix
#'
#' @param x A character vector.
#' @param y A suffix.
#' @returns A character vector with upper case version of the input.
#' @export
add_suffix <- function(x, y) {
  .Call(savvy_add_suffix, x, y)
}

#' Multiply Input By Two
#'
#' @param x An integer vector.
#' @returns An integer vector with values multiplied by 2.
#' @export
times_two_int <- function(x) {
  .Call(savvy_times_two_int, x)
}

#' Multiply Input By Another Input
#'
#' @param x An integer vector.
#' @param y An integer to multiply.
#' @returns An integer vector with values multiplied by `y`.
#' @export
times_any_int <- function(x, y) {
  .Call(savvy_times_any_int, x, y)
}

#' Multiply Input By Two
#'
#' @param x A numeric vector.
#' @returns A numeric vector with values multiplied by 2.
#' @export
times_two_numeric <- function(x) {
  .Call(savvy_times_two_numeric, x)
}

#' Multiply Input By Another Input
#'
#' @param x A real vector.
#' @param y A real to multiply.
#' @returns A real vector with values multiplied by `y`.
#' @export
times_any_numeric <- function(x, y) {
  .Call(savvy_times_any_numeric, x, y)
}

#' Flip Input
#'
#' @param x A logical vector.
#' @returns A logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
flip_logical <- function(x) {
  .Call(savvy_flip_logical, x)
}

#' Or operation
#'
#' @param x A logical vector.
#' @param y A logical value.
#' @returns A logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
or_logical <- function(x, y) {
  .Call(savvy_or_logical, x, y)
}

#' Print the content of list
#'
#' @param x A list vector.
#' @returns `NULL`
#' @export
print_list <- function(x) {
  invisible(.Call(savvy_print_list, x))
}

#' A person with a name
#'
#' @export
Person <- function() {
  e <- new.env(parent = emptyenv())
  self <- .Call(savvy_Person_new)

  e$set_name <- Person_set_name(self)
  e$name <- Person_name(self)
  e$associated_function <- Person_associated_function(self)

  class(e) <- "Person"
  e
}


Person_set_name <- function(self) {
  function(name) {
    invisible(.Call(savvy_Person_set_name, self, name))
  }
}

Person_name <- function(self) {
  function() {
    .Call(savvy_Person_name, self)
  }
}

Person_associated_function <- function(self) {
  function() {
    .Call(savvy_Person_associated_function)
  }
}


