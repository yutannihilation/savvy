# it seems devtools::test() re-compiles the source code with DEBUG=true, so, at
# least on GitHub CI, this test should succeed. When this test is executed on
# local after the package is build with release profile, this fails.
test_that("panic doesn't crash R session", {
  expect_error(must_panic())
})
