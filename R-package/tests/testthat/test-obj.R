test_that("tests for OBJSXP (S4/S7)", {
  # Test S4
  methods::setClass("s4Foo", slots = list(x = "numeric"))
  s4_obj <- methods::new("s4Foo", x = 1)
  expect_s4_class(s4_obj, "s4Foo")

  expect_true(is_obj(s4_obj))
  expect_true(is_obj(unclass(s4_obj)))
  expect_identical(get_obj_class(s4_obj), "s4Foo")
  expect_identical(get_obj_class(unclass(s4_obj)), character(0))

  expect_identical(get_obj_class_typed(s4_obj), "s4Foo")
  expect_identical(get_obj_class_typed(unclass(s4_obj)), character(0))

  # Test non-obj
  expect_false(is_obj(1:3))
  expect_false(is_obj(NULL))
  expect_null(get_obj_class(1:3))
  expect_null(get_obj_class(data.frame(x = 1:3)))

  expect_error(get_obj_class_typed(1:3))
  expect_error(get_obj_class_typed(NULL))

  # Test S7
  skip_if_not_installed("S7")
  S7Foo <- S7::new_class("S7Foo", properties = list(x = S7::class_numeric))
  s7_obj <- S7Foo(x = 1)
  expect_s7_class(s7_obj, S7Foo)

  expect_true(is_obj(s7_obj))
  expect_true(is_obj(unclass(s7_obj)))
  expect_identical(get_obj_class(s7_obj), c("savvyExamples::S7Foo", "S7_object"))
  expect_identical(get_obj_class(unclass(s7_obj)), character(0))

  expect_identical(get_obj_class_typed(s7_obj), c("savvyExamples::S7Foo", "S7_object"))
  expect_identical(get_obj_class_typed(unclass(s7_obj)), character(0))
})
