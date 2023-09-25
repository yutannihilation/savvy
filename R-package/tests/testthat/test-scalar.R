test_that("scalar functions reject non-scalar values and missing values", {
  # no error
  expect_output(scalar_input_int(1L), "1")
  expect_output(scalar_input_real(1.3), "1.3")
  expect_output(scalar_input_logical(FALSE), "false")
  expect_output(scalar_input_string("foo"), "foo")

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

test_that("function can return scalar value", {
  # no error
  expect_equal(scalar_output_int(), 1L)
  expect_equal(scalar_output_real(), 1.3)
  expect_equal(scalar_output_logical(), FALSE)
  expect_equal(scalar_output_string(), "foo")
})

test_that("sum functions", {
  expect_equal(sum_int(1:10), 55L)
  expect_equal(sum_real(c(1, 10, 100, 1000)), 1111)
})
