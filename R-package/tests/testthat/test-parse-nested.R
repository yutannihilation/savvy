test_that("nested Rust files are properly parsed", {
  # should be parsed
  expect_output(fun_mod1(), "foo!")
  expect_output(fun_mod1_1_foo(), "foo!")

  # should not be parsed
  expect_error(fun_mod2())
  expect_error(fun_mod3())
})
