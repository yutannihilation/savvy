test_that("NumericSexp works", {
  # i32 to f64
  expect_equal(
    times_two_numeric_f64(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )

  # f64 to f64
  expect_equal(
    times_two_numeric_f64(c(1.1, NA, 0.0, -1.1, Inf, -Inf)),
    c(2.2, NA, 0.0, -2.2, Inf, -Inf)
  )

  # i32 to i32
  expect_equal(
    times_two_numeric_i32(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )

  # f64 to i32
  expect_equal(
    times_two_numeric_i32(c(1, NA, 0, -1)),
    c(2L, NA, 0L, -2L)
  )

  # error cases
  expect_error(times_two_numeric_i32(Inf)) # infinite
  expect_error(times_two_numeric_i32(2147483648)) # out of i32's range
  expect_error(times_two_numeric_i32(c(1.1, -1.1))) # not integer-ish
})

test_that("NumericSexp can handle 0-length vectors", {
  expect_equal(times_two_numeric_f64(integer(0L)), numeric(0L))
  expect_equal(times_two_numeric_f64(numeric(0L)), numeric(0L))
  expect_equal(times_two_numeric_i32(integer(0L)), integer(0L))
  expect_equal(times_two_numeric_i32(numeric(0L)), integer(0L))
})

test_that("NumericSexp works for usize conversions", {
  # i32 to usize
  expect_equal(
    usize_to_string(c(0L, 10L)),
    c("0", "10")
  )

  # f64 to usize
  expect_equal(
    # 2147483647 = .Machine$integer.max
    usize_to_string(c(0.0, 10.0, 2147483648.0, 9007199254740991)),
    c("0", "10", "2147483648", "9007199254740991")
  )

  # error cases
  expect_error(usize_to_string(NA_integer_))
  expect_error(usize_to_string(NA_real_))
  expect_error(usize_to_string(Inf))
  expect_error(usize_to_string(NaN))
  expect_error(usize_to_string(-1L))
  expect_error(usize_to_string(-1.0))
  expect_error(usize_to_string_scalar(9007199254740992.0))
})


test_that("NumericScalar works", {
  expect_equal(times_two_numeric_f64_scalar(1L), 2)
  expect_equal(times_two_numeric_f64_scalar(1), 2)
  expect_equal(times_two_numeric_f64_scalar(Inf), Inf)
  expect_error(times_two_numeric_f64_scalar(c(1, 2)))
  expect_error(times_two_numeric_f64_scalar(NA_integer_))
  expect_error(times_two_numeric_f64_scalar(NA_real_))
  expect_error(times_two_numeric_f64_scalar("1"))

  expect_equal(times_two_numeric_i32_scalar(1L), 2L)
  expect_equal(times_two_numeric_i32_scalar(1), 2L)
  expect_error(times_two_numeric_i32_scalar(NA_integer_))
  expect_error(times_two_numeric_i32_scalar(NA_real_))
  expect_error(times_two_numeric_i32_scalar(Inf)) # infinite
  expect_error(times_two_numeric_i32_scalar(2147483648)) # out of i32's range
  expect_error(times_two_numeric_i32_scalar(1.1)) # not integer-ish
})

test_that("NumericScalar works for usize conversions", {
  # i32 to usize
  expect_equal(usize_to_string_scalar(0L), "0")
  expect_equal(usize_to_string_scalar(10L), "10")

  # f64 to usize
  expect_equal(usize_to_string_scalar(0.0), "0")
  expect_equal(usize_to_string_scalar(10.0), "10")
  # 2147483647 = .Machine$integer.max
  expect_equal(usize_to_string_scalar(2147483648.0), "2147483648")
  expect_equal(usize_to_string_scalar(9007199254740991), "9007199254740991")

  # error cases
  expect_error(usize_to_string_scalar(NA_integer_))
  expect_error(usize_to_string_scalar(NA_real_))
  expect_error(usize_to_string_scalar(Inf))
  expect_error(usize_to_string_scalar(NaN))
  expect_error(usize_to_string_scalar(-1L))
  expect_error(usize_to_string_scalar(-1.0))
  expect_error(usize_to_string_scalar(9007199254740992.0))
})

test_that("is_numeric() rejects logical (#387)", {
  expect_true(is_numeric(0))
  expect_true(is_numeric(NA_real_))
  expect_true(is_numeric(0L))
  expect_true(is_numeric(NA_integer_))

  expect_false(is_numeric(NA))
  expect_false(is_numeric(NA_character_))
})
