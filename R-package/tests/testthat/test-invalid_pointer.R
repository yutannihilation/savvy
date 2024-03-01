test_that("invalid pointer doesn't clash the session", {
  dir.create("data", showWarnings = FALSE)
  rds_file <- file.path("data", "person.rds")

  if (file.exists(rds_file)) {
    x <- readRDS(rds_file)
    expect_error(x$name(), "invalid pointer")
  } else {
    x <- Person()
    saveRDS(x, rds_file)

    skip("This test is for the first run. Please rerun later.")
  }
})
