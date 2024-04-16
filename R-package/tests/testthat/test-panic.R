test_that("panic doesn't crash R session", {
  expect_error(must_panic())
})
