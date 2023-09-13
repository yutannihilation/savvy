#' @useDynLib unextendr, .registration = TRUE
#' @keywords internal
"_PACKAGE"

#' Convert Input To Upper-Case
#'
#' @param x A character vector.
#' @returns A character vector with upper case version of the input.
#' @export
to_upper <- function(x) {
  .Call(unextendr_to_upper, x)
}

#' Multiply Input By Two
#'
#' @param x An integer vector.
#' @returns An integer vector with values multiplied by 2.
#' @export
times_two_int <- function(x) {
  .Call(unextendr_times_two_int, x)
}

#' Multiply Input By Two
#'
#' @param x A numeric vector.
#' @returns A numeric vector with values multiplied by 2.
#' @export
times_two_numeric <- function(x) {
  .Call(unextendr_times_two_numeric, x)
}

#' Flip Input
#'
#' @param x A logical vector.
#' @returns A logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
flip_logical <- function(x) {
  .Call(unextendr_flip_logical, x)
}

#' Print the content of list
#'
#' @param x A list vector.
#' @returns `NULL`
#' @export
print_list <- function(x) {
  invisible(.Call(unextendr_print_list, x))
}

#' A person with a name
#'
#' @export
Person <- function() {
  e <- new.env(parent = emptyenv())
  self <- .Call(unextendr_Person_new)

  e$set_name <- Person_set_name(self)
  e$name <- Person_name(self)
  e$associated_function <- Person_associated_function(self)

  class(e) <- "Person"
  e
}


Person_set_name <- function(self) {
  function(name) {
    invisible(.Call(unextendr_Person_set_name, self, name))
  }
}

Person_name <- function(self) {
  function() {
    .Call(unextendr_Person_name, self)
  }
}

Person_associated_function <- function(self) {
  function() {
    .Call(unextendr_Person_associated_function)
  }
}



