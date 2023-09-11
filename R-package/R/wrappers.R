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

#' @export
Person <- function() {
  `class<-`(.Call(unextendr_Person_new), "Person")
}

#' @export
person_set_name <- function(self__) {
  UseMethod("person_set_name", self__)
}

#' @export
person_set_name.Person <- function(self__, name) {
  invisible(.Call(unextendr_Person_set_name, self__, name))
}

#' @export
person_name <- function(self__) {
  UseMethod("person_name", self__)
}

#' @export
person_name.Person <- function(self__) {
  .Call(unextendr_Person_name, self__)
}
