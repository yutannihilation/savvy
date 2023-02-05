#' Return a static string.
#'
#' @export
string <- function(x) .Call(`_string`, x)
