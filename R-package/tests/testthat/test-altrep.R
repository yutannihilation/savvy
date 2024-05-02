test_that("altinteger works", {

  ### immutable

  x <- altint()

  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1L, 2L, 3L))

  # after the first materialization, the values visible to R don't change
  tweak_altint(x)
  expect_output(print_altint(x), "MyAltInt([2, 4, 6, 0])", fixed = TRUE)

  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1L, 2L, 3L))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 2L
  expect_equal(x, c(2L, 2L, 3L))

  ### mutable

  x <- altint_mutable()

  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1L, 2L, 3L))

  # the change to values should be reflected
  tweak_altint(x)
  expect_output(print_altint(x), "MyAltIntMutable([2, 4, 6, 0])", fixed = TRUE)

  expect_equal(x[1], 2L) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("2", "4", "6", "0")) # coerce method
  expect_equal(x, c(2L, 4L, 6L, 0L))
  # duplicate method? dataptr method? I'm not sure
  x[1] <- 10L
  expect_equal(x, c(10L, 4L, 6L, 0L))

})

test_that("altreal works", {

  ### immutable

  x <- altlogical()

  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  expect_equal(x, c(TRUE, FALSE, TRUE))

  # after the first materialization, the values visible to R don't change
  tweak_altlogical(x)
  expect_output(print_altlogical(x), "MyAltLogical([false, true, false, false])", fixed = TRUE)

  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  expect_equal(x, c(TRUE, FALSE, TRUE))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- NA
  expect_equal(x, c(NA, FALSE, TRUE))

  ### mutable

  x <- altlogical_mutable()

  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  expect_equal(x, c(TRUE, FALSE, TRUE))

  # the change to values should be reflected
  tweak_altlogical(x)
  expect_output(print_altlogical(x), "MyAltLogicalMutable([false, true, false, false])", fixed = TRUE)

  expect_equal(x[1], FALSE) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("FALSE", "TRUE", "FALSE", "FALSE")) # coerce method
  expect_equal(x, c(FALSE, TRUE, FALSE, FALSE))
  # duplicate method? dataptr method? I'm not sure
  x[1] <- NA
  expect_equal(x, c(NA, TRUE, FALSE, FALSE))

})

test_that("altlogical works", {
  x <- altlogical()
  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  # duplicate method? dataptr method? I'm not sure
  x[1] <- FALSE
  expect_equal(x, c(FALSE, FALSE, TRUE))
})

test_that("altstring works", {
  x <- altstring()
  expect_equal(x[1], "1") # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.integer(x), c(1L, 2L, 3L)) # coerce method
  # duplicate method? dataptr method? I'm not sure
  x[1] <- "foo"
  expect_equal(x, c("foo", "2", "3"))
})
