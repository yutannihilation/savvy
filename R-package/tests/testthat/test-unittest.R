test_that("functions work", {
  expect_equal(
    to_upper(c("a", NA, "A", "\u3042")),
    c("A", NA, "A", "\u3042")
  )
  expect_equal(
    times_two_int(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )
  expect_equal(
    times_two_numeric(c(1.1, NA, 0.0, -1.1, Inf, -Inf)),
    c(2.2, NA, 0.0, -2.2, Inf, -Inf)
  )
  # This cannot handle NA
  # c.f. https://cpp11.r-lib.org/articles/cpp11.html#boolean
  expect_equal(
    flip_logical(c(TRUE, FALSE, NA)),
    c(FALSE, TRUE, TRUE)
  )
})

test_that("structs work", {
  x <- Person()
  expect_s3_class(x, "Person")

  expect_equal(x$name(), "")

  x$set_name("foo")
  expect_equal(x$name(), "foo")

  expect_equal(x$associated_function(), "associated_function")
})
