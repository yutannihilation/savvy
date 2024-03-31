test_that("try_from_iter() works", {
  expect_equal(filter_integer_odd(c(1:10, NA)), c(2L, 4L, 6L, 8L, 10L, NA))
  expect_equal(filter_real_negative(c(1, 0, NA, -1, -2)), c(1, 0, NA))
  expect_equal(filter_logical_duplicates(c(TRUE, TRUE, FALSE, TRUE, FALSE, FALSE, FALSE)), c(TRUE, FALSE, TRUE, FALSE))
  expect_equal(filter_complex_without_im(1 + 1i * c(1, 0, -1, NA)), c(1 + 1i * c(1, -1, NA)))
  expect_equal(filter_string_ascii(c("a", "A", "\u30b9\u30d7\u30e9\u30c8\u30a5\u30fc\u30f3", NA)), c("a", "A", NA))
})
