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
  expect_error(times_two_numeric_i32(Inf))          # infinite
  expect_error(times_two_numeric_i32(2147483648))   # out of i32's range
  expect_error(times_two_numeric_i32(c(1.1, -1.1))) # not integer-ish
})
