test_that("functions work", {
  # character vector
  expect_equal(
    to_upper(c("a", NA, "A", "\u3042")),
    c("A", NA, "A", "\u3042")
  )

  # character vector and scalar
  expect_equal(
    add_suffix(c("a", NA, "A", "\u3042"), "foo"),
    c("a_foo", NA, "A_foo", "\u3042_foo")
  )

  # integer vector
  expect_equal(
    times_two_int(c(1L, NA, 0L, -1L)),
    c(2L, NA, 0L, -2L)
  )

  # integer vector and scalar
  expect_equal(
    times_any_int(c(1L, NA, 0L, -1L), 100L),
    c(100L, NA, 0L, -100L)
  )

  # real vector
  expect_equal(
    times_two_real(c(1.1, NA, 0.0, -1.1, Inf, -Inf)),
    c(2.2, NA, 0.0, -2.2, Inf, -Inf)
  )

  # real vector and scalar
  expect_equal(
    times_any_real(c(1.1, NA, 0.0, -1.1, Inf, -Inf), 100.0),
    c(110.0, NA, 0.0, -110.0, Inf, -Inf)
  )

  # bool vector
  # Note: bool cannot handle NA
  # c.f. https://cpp11.r-lib.org/articles/cpp11.html#boolean
  expect_equal(
    flip_logical(c(TRUE, FALSE, NA)),
    c(FALSE, TRUE, TRUE)
  )

  # Use as_slice_raw() to handle NA
  expect_equal(
    flip_logical_expert_only(c(TRUE, FALSE, NA)),
    c(FALSE, TRUE, NA)
  )

  # bool vector and scalar
  expect_equal(
    or_logical(c(TRUE, FALSE), TRUE),
    c(TRUE, TRUE)
  )

  expect_equal(
    or_logical(c(TRUE, FALSE), FALSE),
    c(TRUE, FALSE)
  )

  expect_equal(
    reverse_bits(as.raw(c(0x0f, 0x00, 0x12))),
    as.raw(c(0xf0, 0x00, 0x48))
  )

  expect_equal(
    reverse_bit_scalar(as.raw(0x12)),
    as.raw(0x48)
  )

  expect_equal(
    list_with_no_values(),
    list(foo = NULL, bar = NULL)
  )

  expect_equal(
    list_with_no_names(),
    list(100L, "cool")
  )

  expect_equal(
    list_with_names_and_values(),
    list(foo = 100L, bar = "cool")
  )
})

test_that("functions can handle 0-length vectors", {
  expect_equal(to_upper(character(0L)),     character(0L))
  expect_equal(times_two_int(integer(0L)),  integer(0L))
  expect_equal(times_two_real(numeric(0L)), numeric(0L))
  expect_equal(flip_logical(logical(0L)),   logical(0L))
  expect_equal(reverse_bits(raw(0L)),       raw(0L))
})

test_that("functions can handle ALTREP", {
  expect_equal(times_two_int(1:10), 1:10 * 2L)
})

test_that("structs work", {
  x <- Person$new()
  expect_s3_class(x, "Person")

  expect_equal(x$name(), "")

  x$set_name("foo")
  expect_equal(x$name(), "foo")

  expect_equal(Person$associated_function(), "associated_function")

  x2 <- Person$new2()
  expect_s3_class(x2, "Person")
  expect_equal(x2$name(), "")
})

test_that("alternative constructor of a struct works", {
  x <- Person$new_with_name("123")
  expect_s3_class(x, "Person")
  expect_equal(x$name(), "123")
})

test_that("function that returns a struct works", {
  # bare function
  x <- external_person_new()
  expect_s3_class(x, "Person")
  x$set_name("foo")
  expect_equal(x$name(), "foo")

  # method
  x2 <- x$another_person() # creates Person2 with copying the name
  expect_s3_class(x2, "Person2")
  expect_equal(x2$name(), "foo")
})

test_that("function that takes a struct works", {
  x <- Person$new()
  x$set_name("foo")

  expect_equal(get_name_external(x), "foo")

  set_name_external(x, "bar")
  expect_equal(get_name_external(x), "bar")
})
