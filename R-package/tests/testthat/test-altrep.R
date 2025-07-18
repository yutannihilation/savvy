test_that("altinteger works", {
  x <- altint()

  expect_equal(x[1], 1L) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(sum(x), 6L) # default sum method
  expect_equal(min(x), 1L) # default min method
  expect_equal(max(x), 3L) # default max method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1L, 2L, 3L))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altint(x)
  expect_output(print_altint(x), "MyAltInt([2, 4, 6, 0])", fixed = TRUE)

  expect_equal(x[1], 2L) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(sum(x), 12L) # default sum method
  expect_equal(min(x), 0L) # default min method
  expect_equal(max(x), 6L) # default max method
  expect_equal(as.character(x), c("2", "4", "6", "0")) # coerce method
  expect_equal(x, c(2L, 4L, 6L, 0L))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 10L
  expect_equal(x, c(10L, 4L, 6L, 0L))
})

test_that("empty altinteger returns Inf", {
  x <- altint_empty()

  expect_equal(length(x), 0L) # length method
  expect_equal(sum(x), 0L) # default sum method
  expect_equal(min(x), Inf) # default min method
  expect_equal(max(x), -Inf) # default max method

  x2 <- altint_na_only()
  expect_equal(sum(x2), NA_integer_) # default sum method
  expect_equal(sum(x2, na.rm = TRUE), 0L) # default sum method
  expect_equal(min(x2, na.rm = TRUE), Inf) # default min method
  expect_equal(max(x2, na.rm = TRUE), -Inf) # default max method
})

test_that("altinteger sum can handle larger number than i32::MAX", {
  x <- altint_toobig()

  expect_equal(sum(x), 2.0 * .Machine$integer.max)
})

test_that("altinteger with custom sum(), min(), and max() works", {
  x <- altint2()

  expect_equal(sum(x), 20)
  expect_equal(min(x), 30)
  expect_equal(max(x), 40)
})

test_that("altreal works", {
  x <- altreal()

  expect_equal(x[1], 1) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(sum(x), 6) # default sum method
  expect_equal(min(x), 1) # default min method
  expect_equal(max(x), 3) # default max method
  expect_equal(as.character(x), c("1", "2", "3")) # coerce method
  expect_equal(x, c(1, 2, 3))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altreal(x)
  expect_output(
    print_altreal(x),
    "MyAltReal([2.0, 4.0, 6.0, 0.0])",
    fixed = TRUE
  )

  expect_equal(x[1], 2) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(sum(x), 12) # default sum method
  expect_equal(min(x), 0) # default min method
  expect_equal(max(x), 6) # default max method
  expect_equal(as.character(x), c("2", "4", "6", "0")) # coerce method
  expect_equal(x, c(2, 4, 6, 0))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- 10
  expect_equal(x, c(10, 4, 6, 0))
})

test_that("empty altreal returns Inf", {
  x <- altreal_empty()

  expect_equal(length(x), 0L) # length method
  expect_equal(sum(x), 0.0) # default sum method
  expect_equal(min(x), Inf) # default min method
  expect_equal(max(x), -Inf) # default max method

  x2 <- altreal_na_only()
  expect_equal(sum(x2), NA_real_) # default sum method
  expect_equal(sum(x2, na.rm = TRUE), 0.0) # default sum method
  expect_equal(min(x2, na.rm = TRUE), Inf) # default min method
  expect_equal(max(x2, na.rm = TRUE), -Inf) # default max method
})

test_that("altreal with custom sum(), min(), and max() works", {
  x <- altreal2()

  expect_equal(sum(x), 20)
  expect_equal(min(x), 30)
  expect_equal(max(x), 40)
})

test_that("altlogical works", {
  x <- altlogical()

  expect_equal(x[1], TRUE) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("TRUE", "FALSE", "TRUE")) # coerce method
  expect_equal(x, c(TRUE, FALSE, TRUE))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altlogical(x)
  expect_output(
    print_altlogical(x),
    "MyAltLogical([false, true, false, false])",
    fixed = TRUE
  )

  expect_equal(x[1], FALSE) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("FALSE", "TRUE", "FALSE", "FALSE")) # coerce method
  expect_equal(x, c(FALSE, TRUE, FALSE, FALSE))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- NA
  expect_equal(x, c(NA, TRUE, FALSE, FALSE))
})

test_that("altraw works", {
  x <- altraw()

  expect_equal(x[1], as.raw(1L)) # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.character(x), c("01", "02", "03")) # coerce method
  expect_equal(x, as.raw(1:3))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altraw(x)
  expect_output(print_altraw(x), "MyAltRaw([2, 3, 4, 2])", fixed = TRUE)

  expect_equal(x[1], as.raw(2L)) # ELT method
  expect_equal(length(x), 4L) # length method
  expect_equal(as.character(x), c("02", "03", "04", "02")) # coerce method
  expect_equal(x, as.raw(c(2L, 3L, 4L, 2L)))

  # duplicate method? dataptr method? I'm not sure
  x[1] <- as.raw(100L)
  expect_equal(x, as.raw(c(100L, 3L, 4L, 2L)))
})

test_that("altstring works", {
  x <- altstring()

  expect_equal(x[1], "1") # ELT method
  expect_equal(length(x), 3L) # length method
  expect_equal(as.numeric(x), c(1, 2, 3)) # coerce method
  expect_equal(x, c("1", "2", "3"))

  # after the first materialization, the values reflect to R's side if invalidate_cache = TRUE
  tweak_altstring(x)
  expect_output(
    print_altstring(x),
    "MyAltString([\"10\", \"20\", \"30\", \"-1\"])",
    fixed = TRUE
  )

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
  expect_output(
    print_altlist(x),
    'MyAltList { one: MyAltInt([2, 4, 6, 0]), two: MyAltString(["a0", "b0", "c0", "-1"]) }',
    fixed = TRUE
  )

  expect_equal(x[[1]], c(2L, 4L, 6L, 0L)) # ELT method
  expect_equal(x[[2]], c("a0", "b0", "c0", "-1")) # ELT method
  expect_equal(length(x), 2L) # length method
  expect_equal(
    x,
    list(one = c(2L, 4L, 6L, 0L), two = c("a0", "b0", "c0", "-1"))
  )

  # duplicate method? dataptr method? I'm not sure
  x[[1]] <- "A"
  expect_equal(x, list(one = "A", two = c("a0", "b0", "c0", "-1")))
})

test_that("get_altrep_body_ref_unchecked() works", {
  x <- altint()
  # The same result of print_altint() is archieved by treating MyAltInt as the external class.
  expect_output(
    print_altint_by_weird_way(x),
    "MyAltInt([1, 2, 3])",
    fixed = TRUE
  )
})
