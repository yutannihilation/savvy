
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
pointer](%22https://en.wikipedia.org/wiki/Tagged_pointer%22) to indicate
the error. This requires to create some wrappers on C’s side to raise an
error after checking the tag, but the advantage is that this doesn’t
need additional allocation and these bit operations should be cheap.

See [my blog
post](https://yutani.rbind.io/post/dont-panic-we-can-unwind/) for more
details.

### Read-only and writable versions of wrappers

To consider safety first, Rust should not modify the memory allocated by
R, and vice versa. So, this framework distinguishes the read-only (or
external) data and writable (or owned) data. While this costs some
additional memory allocation, this is also good in that we can skip
protecting the external data.

This design comes from [cpp11’s
`writable`](https://cpp11.r-lib.org/articles/motivations.html#copy-on-write-semantics).

### Do we really want embedded usages?

I’ve been wondering if it would be simpler if extendr give up supporting
embedded usages. Yes, extendr is not only about R packages. It also
provides functionalities to embed R in Rust like [this
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

x <- Person()

x$set_name("たかし")
x$name()
#> [1] "たかし"
```

## Rust API

<table style="width:98%;">
<colgroup>
<col style="width: 16%" />
<col style="width: 23%" />
<col style="width: 27%" />
<col style="width: 30%" />
</colgroup>
<thead>
<tr class="header">
<th>R type</th>
<th>Read-only version</th>
<th>Writable version</th>
<th>Note</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>SEXP</code></td>
<td><code>Sxp</code></td>
<td>-</td>
<td></td>
</tr>
<tr class="even">
<td><code>INTSXP</code></td>
<td><code>IntegerSxp</code></td>
<td><code>OwnedIntegerSxp</code></td>
<td></td>
</tr>
<tr class="odd">
<td><code>REALSXP</code></td>
<td><code>RealSxp</code></td>
<td><code>OwnedRealSxp</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>LGLSXP</code></td>
<td><code>LogicalSxp</code></td>
<td><code>OwnedLogicalSxp</code></td>
<td><ul>
<li>cannot handle <code>NA</code></li>
</ul></td>
</tr>
<tr class="odd">
<td><code>STRSXP</code></td>
<td><code>StringSxp</code></td>
<td><code>OwnedStringSxp</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>VECSXP</code></td>
<td><code>ListSxp</code></td>
<td><code>OwnedListSxp</code></td>
<td></td>
</tr>
<tr class="odd">
<td><code>EXTPTRSXP</code></td>
<td>-<a href="#fn1" class="footnote-ref" id="fnref1"
role="doc-noteref"><sup>1</sup></a></td>
<td><code>ExternalPointerSxp</code></td>
<td></td>
</tr>
</tbody>
</table>
<section id="footnotes" class="footnotes footnotes-end-of-document"
role="doc-endnotes">
<hr />
<ol>
<li id="fn1"><p>This framework handles only <code>EXTPTRSXPs</code>
created by this framework. While this is an “external” pointer to R,
it’s internal from the viewpoint of Rust.<a href="#fnref1"
class="footnote-back" role="doc-backlink">↩︎</a></p></li>
</ol>
</section>

## References

- <https://notchained.hatenablog.com/entry/2023/01/29/163013> (日本語)
