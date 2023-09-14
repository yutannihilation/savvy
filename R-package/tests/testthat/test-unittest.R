test_that("functions work", {
  expect_equal(
    to_upper(c("a", NA, "A", "\u3042")),
    c("A", NA, "A", "\u3042")
  )

  expect_equal(
    add_suffix(c("a", NA, "A", "\u3042"), "foo"),
    c("a_foo", NA, "A_foo", "\u3042_foo")
  )

  expect_equal(
    times_two_int(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )

  expect_equal(
    times_any_int(c(1L, NA, 0L, -1L), 100L),
    c(100L, NA, 0L, -100L)
  )

  expect_equal(
    times_two_numeric(c(1.1, NA, 0.0, -1.1, Inf, -Inf)),
    c(2.2, NA, 0.0, -2.2, Inf, -Inf)
  )

  expect_equal(
    times_any_numeric(c(1.1, NA, 0.0, -1.1, Inf, -Inf), 100.0),
    c(110.0, NA, 0.0, -110.0, Inf, -Inf)
  )

  # This cannot handle NA
  # c.f. https://cpp11.r-lib.org/articles/cpp11.html#boolean
  expect_equal(
    flip_logical(c(TRUE, FALSE, NA)),
    c(FALSE, TRUE, TRUE)
  )

  expect_equal(
    or_logical(c(TRUE, FALSE), TRUE),
    c(TRUE, TRUE)
  )

  expect_equal(
    or_logical(c(TRUE, FALSE), FALSE),
    c(TRUE, FALSE)
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
