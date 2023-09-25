test_that("error handling works", {
  # check if a Rust objects are dropped properly
  expect_snapshot(safe_stop(), error = TRUE)

  expect_error(raise_error(), "This is my custom error")
})
