test_that("raw identifiers are treated correctly", {
  expect_no_error(fn(TRUE))
  expect_no_error(struct$fn(TRUE))
  expect_no_error(struct$new())
  expect_no_error(print(Enum$enum))
})
