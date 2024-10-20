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
  expect_equal(scalar_output_int2(), 1L)
  expect_equal(scalar_output_real(), 1.3)
  expect_equal(scalar_output_real2(), 1.3)
  expect_equal(scalar_output_complex(), 1.0 + 1.0i)
  expect_equal(scalar_output_complex2(),  1.0 + 1.0i)
  expect_equal(scalar_output_logical(), FALSE)
  expect_equal(scalar_output_logical2(), FALSE)
  expect_equal(scalar_output_string(), "foo")
  expect_equal(scalar_output_string2(), "foo")
})

test_that("sum functions", {
  expect_equal(sum_int(1:10), 55L)
  expect_equal(sum_real(c(1, 10, 100, 1000)), 1111)
})

test_that("conversion from vectors", {
  expect_equal(rep_int_vec(3L), c(0L, 0L, 0L))
  expect_equal(rep_int_slice(3L), c(0L, 0L, 0L))
  expect_equal(rep_real_vec(3L), c(0, 0, 0))
  expect_equal(rep_real_slice(3L), c(0, 0, 0))
  expect_equal(rep_bool_vec(3L), c(TRUE, TRUE, TRUE))
  expect_equal(rep_bool_slice(3L), c(TRUE, TRUE, TRUE))
  expect_equal(rep_str_vec(3L), c("foo", "foo", "foo"))
  expect_equal(rep_str_slice(3L), c("foo", "foo", "foo"))
})

test_that("user-defined structs", {
  expect_error(get_name_external(NULL))
  x <- Person$new()
  class(x) <- "foo"
  expect_error(get_name_external(x))

  # cannot be modified
  expect_error(Person$"aaa" <- "aaa")
  expect_error(Person[["aaa"]] <- "aaa")
})
