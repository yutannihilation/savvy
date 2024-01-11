test_that("Getting attributes works", {
  cl <- c("foo", "bar", "baz")
  no_class <- 1:10
  with_class <- `class<-`(no_class, cl)
  expect_equal(get_class_int(no_class), NULL)
  expect_equal(get_class_int(with_class), cl)

  no_names <- 1:26
  with_names <- setNames(no_names, LETTERS)
  expect_equal(get_names_int(no_names), rep("", 26))
  expect_equal(get_names_int(with_names), LETTERS)

  expect_equal(get_dim_int(1L), NULL)
  expect_equal(get_dim_int(matrix(1:12, nrow = 3L)), c(3L, 4L))
})
