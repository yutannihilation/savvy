# Taken from https://github.com/pola-rs/r-polars/issues/851#issuecomment-1971551241
test_that("invalid pointer doesn't clash the session", {
  rds_file <- tempfile(fileext = ".rds")

  x <- Person()
  saveRDS(x, rds_file)

  x <- readRDS(rds_file)
  expect_error(x$name(), "Invalid external pointer")
})
