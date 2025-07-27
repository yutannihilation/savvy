test_that("is_scalar_na() works", {
  expect_true(is_scalar_na(NA))
  expect_true(is_scalar_na(NA_character_))
  expect_true(is_scalar_na(NA_real_))
  expect_true(is_scalar_na(NA_integer_))

  expect_false(is_scalar_na(c(NA, NA)))
  expect_false(is_scalar_na(c(NA, NA)))
  expect_false(is_scalar_na(NULL))
})
