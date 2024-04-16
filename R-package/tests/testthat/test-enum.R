test_that("enum works", {
  a <- FooEnum$A
  expect_s3_class(a, "FooEnum")
  expect_output(a$print(), "A")
  expect_output(print_foo_enum(a), "A")
  expect_output(print_foo_enum_ref(a), "A")

  # print method
  expect_output(print(FooEnum$A), "FooEnum::A")
  expect_output(print(FooEnum$B), "FooEnum::B")

  # corrupt
  FooEnum$B$.ptr <- 3L
  expect_error(print(FooEnum$B))
})
