test_that("environment", {
  e1 <- new.env(parent = emptyenv())
  e1$a <- "foo"

  expect_true(var_exists_in_env("a", e1))
  expect_false(var_exists_in_env("b", e1))

  expect_equal(get_var_in_env("a", e1), "foo")
  expect_error(get_var_in_env("b", e1))

  # doesn't climb up the parent environments
  e2 <- new.env(parent = e1)
  expect_false(var_exists_in_env("a", e2))

  set_var_in_env("c", 100L, e1)
  expect_equal(e1$c, 100L)
  # overwrite
  set_var_in_env("c", 300L, e1)
  expect_equal(e1$c, 300L)

  # global env
  .GlobalEnv$global_obj <- "ABC"
  expect_equal(get_var_in_env("global_obj"), "ABC")

  # a promise is forced
  e3 <- new.env(parent = emptyenv())
  delayedAssign("d", "lazy", assign.env = e3)
  expect_equal(get_var_in_env("d", e3), "lazy")

  # an error while forcing a promise is propagated
  delayedAssign("e", stop("boom"), assign.env = e3)
  expect_error(get_var_in_env("e", e3), "boom")
})
