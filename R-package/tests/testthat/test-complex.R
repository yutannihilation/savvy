test_that("complex works", {
  expect_equal(new_complex(3L), rep(0+0i, 3))
  expect_equal(first_complex(1:3 + 1i * (3:1)), 1+3i)
  expect_equal(abs_complex(c(3+4i, NA, 1+1i)), c(5, NA, sqrt(2)))
})
