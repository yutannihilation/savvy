test_that("Getting attributes works", {
  cl <- c("foo", "bar", "baz")
  no_class <- 1:10
  with_class <- `class<-`(no_class, cl)
  expect_equal(get_class_int(no_class), NULL)
  expect_equal(get_class_int(with_class), cl)

  no_names <- 1:26
  with_names <- setNames(no_names, LETTERS)
  expect_equal(get_names_int(no_names), NULL)
  expect_equal(get_names_int(with_names), LETTERS)

  expect_equal(get_dim_int(1L), NULL)
  expect_equal(get_dim_int(matrix(1:12, nrow = 3L)), c(3L, 4L))

  attr <- c("foo", "bar", "baz")
  no_attr <- 1:10
  with_attr <- `attr<-`(no_attr, "foo", attr)
  expect_equal(get_attr_int(no_attr, "foo"), NULL)
  expect_equal(get_attr_int(with_attr, "foo"), attr)
})

test_that("Setting attributes works", {
  expect_s3_class(set_class_int(), c("foo", "bar"))
  expect_equal(names(set_names_int()), c("foo", "bar"))
  expect_equal(dim(set_dim_int()), c(2L, 3L))

  attr <- list(a = 10)
  x <- set_attr_int("foo", attr)
  expect_equal(attr(x, "foo"), attr)
})
