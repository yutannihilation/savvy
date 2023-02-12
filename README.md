
<!-- README.md is generated from README.Rmd. Please edit that file -->

# unextendr

<!-- badges: start -->
<!-- badges: end -->

## References

- <https://notchained.hatenablog.com/entry/2023/01/29/163013> (日本語)

## Functions

``` r
library(unextendr)

to_upper("a")
#> [1] "A"
times_two_int(1L)
#> [1] 2
times_two_numeric(1.1)
#> [1] 2.2
```

## Random thoughts

### Error Handling

This framework uses [tagged
pointer](https://en.wikipedia.org/wiki/Tagged_pointer) to indicate the
error. This requires some wrappers, but the advantage is that we don’t
need additional information.

### Protection

I implemented the doubly-linked list method, which [cpp11
uses](https://cpp11.r-lib.org/articles/internals.html#protection). Now
I’m not sure if this fits extendr; probably its current implementation
aims for parallel processing, so it needs a hashmap to prevent
collisions. But, I think it’s not a good idea to use R’s C API
parallelly anyway, so this should be probably enough.
