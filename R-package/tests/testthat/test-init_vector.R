test_that("Owned*Sexp is properly initialized", {
  expect_equal(new_real(3L), c(0.0, 0.0, 0.0))
  expect_equal(new_int(3L), c(0, 0, 0))
  expect_equal(new_bool(3L), c(FALSE, FALSE, FALSE))
})
