test_that("functions works", {
  x <- list(a = 1L, b = 2.0, c = "foo")
  expect_equal(do_call(list, x, .GlobalEnv), x)
  expect_equal(do_call(function(...) list(...), x, .GlobalEnv), x)

  expect_equal(call_with_args(list, .GlobalEnv), x)
  expect_equal(call_with_args(function(...) list(...), .GlobalEnv), x)
})
