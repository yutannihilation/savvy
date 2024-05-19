test_that("environment", {
  e1 <- new.env(parent = emptyenv())
  e1$a <- "foo"

  expect_true(var_exists_in_env(e1, "a"))
  expect_false(var_exists_in_env(e1, "b"))

  expect_equal(get_var_in_env(e1, "a"), "foo")
  expect_error(get_var_in_env(e1, "b"))

  # doesn't climb up the parent environments
  e2 <- new.env(parent = e1)
  expect_false(var_exists_in_env(e2, "a"))

  set_var_in_env(e1, "c", 100L)
  expect_equal(e1$c, 100L)
  # overwrite
  set_var_in_env(e1, "c", 300L)
  expect_equal(e1$c, 300L)
})
