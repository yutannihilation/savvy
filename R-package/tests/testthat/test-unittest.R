test_that("functions work", {
  # character vector
  expect_equal(
    to_upper(c("a", NA, "A", "\u3042")),
    c("A", NA, "A", "\u3042")
  )

  # character vector and scalar
  expect_equal(
    add_suffix(c("a", NA, "A", "\u3042"), "foo"),
    c("a_foo", NA, "A_foo", "\u3042_foo")
  )

  # integer vector
  expect_equal(
    times_two_int(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )

  # integer vector and scalar
  expect_equal(
    times_any_int(c(1L, NA, 0L, -1L), 100L),
    c(100L, NA, 0L, -100L)
  )

  # real vector
  expect_equal(
    times_two_numeric(c(1.1, NA, 0.0, -1.1, Inf, -Inf)),
    c(2.2, NA, 0.0, -2.2, Inf, -Inf)
  )

  # real vector and scalar
  expect_equal(
    times_any_numeric(c(1.1, NA, 0.0, -1.1, Inf, -Inf), 100.0),
    c(110.0, NA, 0.0, -110.0, Inf, -Inf)
  )

  # bool vector
  # Note: bool cannot handle NA
  # c.f. https://cpp11.r-lib.org/articles/cpp11.html#boolean
  expect_equal(
    flip_logical(c(TRUE, FALSE, NA)),
    c(FALSE, TRUE, TRUE)
  )

  # bool vector and scalar
  expect_equal(
    or_logical(c(TRUE, FALSE), TRUE),
    c(TRUE, TRUE)
  )

  expect_equal(
    or_logical(c(TRUE, FALSE), FALSE),
    c(TRUE, FALSE)
  )
})

test_that("functions can handle ALTREP", {
  expect_equal(times_two_int(1:10), 1:10 * 2L)
})

test_that("scalar functions reject non-scalar values and missing values", {
  # no error
  expect_no_error(scalar_input_int(1L))
  expect_no_error(scalar_input_real(1.0))
  expect_no_error(scalar_input_logical(TRUE))
  expect_no_error(scalar_input_str("foo"))

  # error
  expect_error(scalar_input_int(1:10))
  expect_error(scalar_input_real(c(1, 2)))
  expect_error(scalar_input_logical(c(TRUE, FALSE)))
  expect_error(scalar_input_str(c("foo", "bar")))

  expect_error(scalar_input_int(NA_integer_))
  expect_error(scalar_input_real(NA_real_))
  expect_error(scalar_input_logical(NA))
  expect_error(scalar_input_str(NA_character_))
})

test_that("NA scalars are rejected", {
  expect_error(add_suffix("", NA_character_))
  expect_error(times_any_int(0L, NA_integer_))
  expect_error(times_any_numeric(0, NA_real_))
  expect_error(or_logical(TRUE, NA))
})

test_that("structs work", {
  x <- Person()
  expect_s3_class(x, "Person")

  expect_equal(x$name(), "")

  x$set_name("foo")
  expect_equal(x$name(), "foo")

  expect_equal(x$associated_function(), "associated_function")
})
