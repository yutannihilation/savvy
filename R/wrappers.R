#' Exported Functions
#'
#' @rdname wrappers
#' @export
to_upper <- function(x) {
  .Call(unextendr_to_upper, x)
}

# #' @rdname wrappers
# #' @export
# preserve_list <- function() .Call(unextendr_preserve_list)

#' @rdname wrappers
#' @export
times_two_int <- function(x) {
  .Call(unextendr_times_two_int, x)
}

#' @rdname wrappers
#' @export
times_two_numeric <- function(x) {
  .Call(unextendr_times_two_numeric, x)
}
