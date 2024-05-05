test_that("error handling works", {
  # check if a Rust objects are dropped properly
  expect_snapshot(safe_stop(), error = TRUE)
  expect_equal(get_foo_value(), 0L) # drop is properly done

  expect_warning(safe_warn())
  withr::with_options(list(warn = 2),
    expect_snapshot(safe_warn(), error = TRUE)
  )
  expect_equal(get_foo_value(), 0L) # drop is properly done

  expect_error(raise_error(), "This is my custom error")
})
