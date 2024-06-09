test_that("enum works", {
  a <- FooEnum$A
  expect_s3_class(a, "FooEnum")
  expect_output(a$print(), "A")
  expect_output(print_foo_enum(a), "A")
  expect_output(print_foo_enum_ref(a), "A")

  # print method
  expect_output(print(FooEnum$A), "FooEnum::A")
  expect_output(print(FooEnum$B), "FooEnum::B")

  # Reject invalid specifications
  expect_error(FooEnum$C)
  expect_error(FooEnum[["C"]])
  expect_error(FooEnum[[1]])

  # cannot be modified in a usual way
  expect_error(FooEnum$C <- "C")
  expect_error(FooEnum[["C"]] <- "C")

  # corrupt
  assign(".ptr", 3L, envir = FooEnum$B)
  expect_error(print(FooEnum$B))
})
