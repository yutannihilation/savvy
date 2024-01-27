test_that("functions works", {
  x <- list(a = 1L, b = 2.0, c = "foo")
  expect_equal(do_call(list, x), x)
  expect_equal(do_call(function(...) list(...), x), x)

  # handle 0-length argument
  expect_equal(do_call(list, list()), list())

  expect_equal(call_with_args(list), x)
  expect_equal(call_with_args(function(...) list(...)), x)
})
