test_that("optional arg works", {
  expect_equal(default_value_scalar(10L), 10L)
  expect_equal(default_value_scalar(), -1L)
  expect_equal(default_value_vec(1:10), 55L)
  expect_equal(default_value_vec(), -1L)

  expect_equal(FooWithDefault$default_value_associated_fn(10L), 10L)
  expect_equal(FooWithDefault$default_value_associated_fn(), -1L)

  x <- FooWithDefault$new(-100L)
  expect_equal(x$default_value_method(10L), 10L)
  expect_equal(x$default_value_method(), -100L)

  expect_equal(default_value_struct(x), -100L)
  expect_equal(default_value_struct(), -1L)

  expect_equal(default_value_enum(FooEnum$A), 1L)
  expect_equal(default_value_enum(), -1L)
})
