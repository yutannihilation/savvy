test_that("array example works", {
  m <- matrix(as.numeric(1:24), ncol = 3)
  expect_snapshot(print_array(m))

  a <- array(as.numeric(1:24), dim = c(2, 3, 4))
  expect_snapshot(print_array(a))
})
