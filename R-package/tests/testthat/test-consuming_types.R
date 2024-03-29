test_that("consuming types work", {
  a <- Value$new(1L)
  b <- Value$new(2L)
  expect_equal(a$get(), 1L)
  expect_equal(b$get(), 2L)

  # not consumed
  x1 <- ValuePair$new_copy(a, b)
  expect_snapshot(x1$print())

  # since they are not consumed, they still can return value
  expect_equal(a$get(), 1L)
  expect_equal(b$get(), 2L)

  # consumed
  x2 <- ValuePair$new(a, b)
  expect_snapshot(x2$print())

  # since they are consumed, this returns an error
  expect_error(a$get())
  expect_error(b$get())

  # method

  a3 <- Value$new(10L)
  b3 <- Value$new(20L)
  x3 <- a3$pair(b3)
  expect_snapshot(x3$print())
  expect_error(a3$get())
  expect_error(b3$get())

  # bare function

  a4 <- Value$new(10L)
  b4 <- Value$new(20L)
  x4 <- new_value_pair(a4, b4)
  expect_snapshot(x4$print())
  expect_error(a4$get())
  expect_error(b4$get())
})
