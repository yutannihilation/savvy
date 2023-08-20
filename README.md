
<!-- README.md is generated from README.Rmd. Please edit that file -->

# unextendr

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/unextendr/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/yutannihilation/unextendr/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

## What’s this?

This is… nothing. I’m just re-inventing the wheel only to have slightly
better understanding about what [extendr](https://extendr.github.io/)
does.

## Random thoughts

### Error Handling

This framework uses [tagged
pointer](https://en.wikipedia.org/wiki/Tagged_pointer) to indicate the
error. This requires some wrappers, but the advantage is that we don’t
need to pass additional data and these bit operations should be cheap.

See [extendr/extendr#278](https://github.com/extendr/extendr/issues/278)
for more discussion.

### Protection

I implemented the doubly-linked list method, which [cpp11
uses](https://cpp11.r-lib.org/articles/internals.html#protection). Now
I’m not sure if this fits extendr; probably its current implementation
aims for parallel processing, so it needs a hashmap to prevent
collisions. But, I think it’s not a good idea to use R’s C API
concurrently anyway, so this should be probably enough.

### Read-only and writable versions of wrappers

cpp11 provides the read-only by default, and [the writable
version](https://cpp11.r-lib.org/articles/motivations.html#copy-on-write-semantics)
as an option.It seems a good idea to distinguish the external SEXPs and
the “owned” SEXPs because we have control, when to protect and
unprotect, only over the latter one.

### Do we really want embedded usages?

Regarding the concurrency, I’m wondering if it would be simpler if
extendr give up supporting embedded usages. Yes, extendr is not only
about R packages. It also provides functionalities to embed R in Rust
like [this
one](https://github.com/yutannihilation/extendr-tide-api-server-example).
This is great, but, on the other hand, this means we have to care a lot
of things. But, how careful we try to be, concurrency is a tough job.

A lot of tests have failed because of various problems related to
concurrency. Every time we encountered such a failure, we place
`single_threaded()` here and there. But, could all of them really happen
inside an R package? If extendr gives up supporting embedded usages, can
our life be simpler a bit?

### Generate bindings by CLI

Extendr embeds the functionality to generate the R bindings and call
entries. But, might it be easier to generate them by using an external
CLI? I actually need this because I need to generate C code to handle
the errors on C’s side.

#### Usage

``` console
Generate C bindings and R bindings for a Rust library

Usage: unextendr-bindgen.exe [COMMAND]

Commands:
  c-header  Generate C header file
  c-impl    Generate C implementation for init.c
  r-impl    Generate R wrapper functions
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` sh
cargo run --manifest-path ./unextendr-bindgen/Cargo.toml -- r-impl ./R-package/src/rust/src/lib.rs > ./R-package/R/wrappers.R
cargo run --manifest-path ./unextendr-bindgen/Cargo.toml -- c-impl ./R-package/src/rust/src/lib.rs > ./R-package/src/init
cargo run --manifest-path ./unextendr-bindgen/Cargo.toml -- c-header ./R-package/src/rust/src/lib.rs > ./R-package/src/rust/api.h
```

## Crates

- `unextendr`: a simple wrapper around R’s C API
- `unextendr-bindgen`: a crate for generating C and R bindings from Rust
  code
- `unextendr-macro`: a crate for `#[unextendr]` macro, which is powered
  by `unextendr-bindgen`

## Example functions

A toy example R package can be found in `R-package/` directory.

``` r
library(unextendr)

to_upper(c("a", NA, "A", "座布団一枚"))
#> [1] "A"          NA           "A"          "座布団一枚"

times_two_int(c(1L, NA, 100L, 0L, -1L))
#> [1]   2  NA 200   0  -2

times_two_numeric(c(1.1, NA, 0.0, Inf, -Inf))
#> [1]  2.2   NA  0.0  Inf -Inf

flip_logical(c(TRUE, FALSE, NA))
#> [1] FALSE  TRUE  TRUE

print_list(list(1:10, a = letters, b = c(TRUE, FALSE), `たかし` = list(), D = NULL))
#> (no name): integer [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
#> a: character [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z]
#> b: logical [TRUE, FALSE]
#> たかし: list
#> D: NULL
```

## TODOs

- [ ] Support Attribute and names
- [ ] Support list
- [ ] Support ALTREP
- [x] Use proc-macro
- [x] `R_UnwindProtect()`

## References

- <https://notchained.hatenablog.com/entry/2023/01/29/163013> (日本語)
