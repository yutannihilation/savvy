test_that("altinteger works", {
  x <- altint()
  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  # duplicate method? dataptr method? I'm not sure
  x[1] <- 2L
  expect_equal(x, c(2L, 2L, 3L))
})

test_that("altreal works", {
  x <- altreal()
  expect_equal(x[1], 1) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  # duplicate method? dataptr method? I'm not sure
  x[1] <- 2
  expect_equal(x, c(2, 2, 3))
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
