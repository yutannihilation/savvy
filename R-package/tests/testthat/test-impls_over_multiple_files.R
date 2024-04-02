test_that("multiplication works", {
  v <- Value$new(1L)
  expect_equal(v$get2(), 1L)
})
