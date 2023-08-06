#' Exported Functions
#'
#' @rdname wrappers
#' @param x A character vector.
#' @returns A character vector with upper case version of the input.
#' @export
to_upper <- function(x) {
  .Call(unextendr_to_upper, x)
}

#' @rdname wrappers
#' @param x An integer vector.
#' @returns An integer vector with values multiplied by 2.
#' @export
times_two_int <- function(x) {
  .Call(unextendr_times_two_int, x)
}

#' @rdname wrappers
#' @param x A numeric vector.
#' @returns A numeric vector with values multiplied by 2.
#' @export
times_two_numeric <- function(x) {
  .Call(unextendr_times_two_numeric, x)
}

#' @rdname wrappers
#' @param x An logical vector.
#' @returns An logical vector with filled values (`NA` is converted to `TRUE`).
#' @export
flip_logical <- function(x) {
  .Call(unextendr_flip_logical, x)
}
