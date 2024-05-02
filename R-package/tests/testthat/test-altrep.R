test_that("altinteger works", {
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

})

test_that("altreal works", {
  x <- altreal()

  expect_equal(x[1], 1) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1, 2, 3))

  # after the first materialization, the values visible to R don't change
  tweak_altreal(x)
  expect_output(print_altreal(x), "MyAltReal([2.0, 4.0, 6.0, 0.0])", fixed = TRUE)

  expect_equal(x[1], 1) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1, 2, 3))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 2
  expect_equal(x, c(2, 2, 3))
})

test_that("altlogical works", {
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
})

test_that("altstring works", {
  x <- altstring()

  expect_equal(x[1], "1") # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.numeric(x), c(1, 2, 3)) # coerce method
  expect_equal(x, c("1", "2", "3"))

  # after the first materialization, the values visible to R don't change
  tweak_altstring(x)
  expect_output(print_altstring(x), "MyAltString([\"10\", \"20\", \"30\", \"-1\"])", fixed = TRUE)

  expect_equal(x[1], "1") # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.numeric(x), c(1, 2, 3)) # coerce method
  expect_equal(x, c("1", "2", "3"))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- "A"
  expect_equal(x, c("A", "2", "3"))
})
