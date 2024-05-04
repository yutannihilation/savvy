test_that("altinteger works", {
  x <- altint()

  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1L, 2L, 3L))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altint(x)
  expect_output(print_altint(x), "MyAltInt([2, 4, 6, 0])", fixed = TRUE)

  expect_equal(x[1], 2L) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("2", "4", "6", "0")) # coerce method
  expect_equal(x, c(2L, 4L, 6L, 0L))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 10L
  expect_equal(x, c(10L, 4L, 6L, 0L))

})

test_that("altreal works", {
  x <- altreal()

  expect_equal(x[1], 1) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1, 2, 3))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altreal(x)
  expect_output(print_altreal(x), "MyAltReal([2.0, 4.0, 6.0, 0.0])", fixed = TRUE)

  expect_equal(x[1], 2) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("2", "4", "6", "0")) # coerce method
  expect_equal(x, c(2, 4, 6, 0))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 10
  expect_equal(x, c(10, 4, 6, 0))
})

test_that("altlogical works", {
  x <- altlogical()

  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  expect_equal(x, c(TRUE, FALSE, TRUE))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altlogical(x)
  expect_output(print_altlogical(x), "MyAltLogical([false, true, false, false])", fixed = TRUE)

  expect_equal(x[1], FALSE) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("FALSE", "TRUE", "FALSE", "FALSE")) # coerce method
  expect_equal(x, c(FALSE, TRUE, FALSE, FALSE))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- NA
  expect_equal(x, c(NA, TRUE, FALSE, FALSE))
})

test_that("altstring works", {
  x <- altstring()

  expect_equal(x[1], "1") # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.numeric(x), c(1, 2, 3)) # coerce method
  expect_equal(x, c("1", "2", "3"))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altstring(x)
  expect_output(print_altstring(x), "MyAltString([\"10\", \"20\", \"30\", \"-1\"])", fixed = TRUE)

  expect_equal(x[1], "10") # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.numeric(x), c(10, 20, 30, -1)) # coerce method
  expect_equal(x, c("10", "20", "30", "-1"))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- "A"
  expect_equal(x, c("A", "20", "30", "-1"))
})

test_that("altlist works", {
  x <- altlist()

  expect_equal(x[[1]], c(1L, 2L, 3L)) # ELT method
  expect_equal(x[[2]], c("a", "b", "c")) # ELT method
  expect_equal(length(x), 2L) # length method
  expect_equal(x, list(one = c(1L, 2L, 3L), two = c("a", "b", "c")))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altlist(x)
  expect_output(print_altlist(x), 'MyAltList { one: MyAltInt([2, 4, 6, 0]), two: MyAltString(["a0", "b0", "c0", "-1"]) }', fixed = TRUE)

  expect_equal(x[[1]], c(2L, 4L, 6L, 0L)) # ELT method
  expect_equal(x[[2]], c("a0", "b0", "c0", "-1")) # ELT method
  expect_equal(length(x), 2L) # length method
  expect_equal(x, list(one = c(2L, 4L, 6L, 0L), two = c("a0", "b0", "c0", "-1")))

  # duplicate method? dataptr method? I'm not sure
  x[[1]] <- "A"
  expect_equal(x, list(one = "A", two =  c("a0", "b0", "c0", "-1")))
})

test_that("get_altrep_body_ref_unchecked() works", {
  x <- altint()
  # The same result of print_altint() is archieved by treating MyAltInt as the external class.
  expect_output(print_altint_by_weird_way(x), "MyAltInt([1, 2, 3])", fixed = TRUE)
})
