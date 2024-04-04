test_that("enum works", {
  a <- FooEnum$A
  expect_s3_class(a, "FooEnum")
  expect_output(a$print(), "A")
  expect_output(print_foo_enum(a), "A")
  expect_output(print_foo_enum_ref(a), "A")
})
