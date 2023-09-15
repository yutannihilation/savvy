
<!-- README.md is generated from README.Rmd. Please edit that file -->

# Savvy - A safe, but unfriendly, extension interface using Rust.

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml)

<!-- badges: end -->

## What’s this?

This is nothing but my personal challenge to re-invent the wheel in
order to get better understanding about what
[extendr](https://extendr.github.io/) does. While this is usable,
ergonomics is not included. Please use extendr.

### Why savvy?

In Japanese, “Rust” is pronounced as `sàbí`(錆). Since the sound is
similar, and this framework is intended to be used by R-API-savvy
people, I chose this name.

## Basic usage

As you can see, this framework is unfriendly in that this requires more
explicit operations than extendr.

- The function’s arguments must be savvy types (`...Sxp`). Scalar types
  (`i32`, `f64`, `bool`, and `&str`) are also allowed, though.
- The function’s return type must be either `savvy::Result<savvy::SEXP>`
  or `()`.
- Unlike extendr, savvy doesn’t take care of the output conversion. You
  have to create a new SEXP object by `Owned...Sxp::new()` and set
  values by `set_elt()` one by one (`.iter_mut()` might be provided for
  int and real, but string is not the case because the internal
  representation is not `[&str]`).

``` rust
#[savvy]
fn to_upper(x: StringSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na());
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str());
    }

    Ok(out.into())
}
```

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

This idea comes from [cpp11’s
`writable`](https://cpp11.r-lib.org/articles/motivations.html#copy-on-write-semantics),
but savvy’s design is less convenient for educational purposes.

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

Usage: savvy-bindgen.exe [COMMAND]

Commands:
  c-header  Generate C header file
  c-impl    Generate C implementation for init.c
  r-impl    Generate R wrapper functions
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` sh
cargo run --manifest-path ./savvy-bindgen/Cargo.toml -- r-impl ./R-package/src/rust/src/lib.rs > ./R-package/R/wrappers.R
cargo run --manifest-path ./savvy-bindgen/Cargo.toml -- c-impl ./R-package/src/rust/src/lib.rs > ./R-package/src/init
cargo run --manifest-path ./savvy-bindgen/Cargo.toml -- c-header ./R-package/src/rust/src/lib.rs > ./R-package/src/rust/api.h
```

## Crates

- `savvy`: a simple wrapper around R’s C API
- `savvy-bindgen`: a crate for generating C and R bindings from Rust
  code
- `savvy-macro`: a crate for `#[savvy]` macro, which is powered by
  `savvy-bindgen`

## Example functions

A toy example R package can be found in `R-package/` directory.

``` r
library(savvy)

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
<td><code>INTSXP</code></td>
<td><code>IntegerSxp</code></td>
<td><code>OwnedIntegerSxp</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>REALSXP</code></td>
<td><code>RealSxp</code></td>
<td><code>OwnedRealSxp</code></td>
<td></td>
</tr>
<tr class="odd">
<td><code>LGLSXP</code></td>
<td><code>LogicalSxp</code></td>
<td><code>OwnedLogicalSxp</code></td>
<td><ul>
<li>cannot handle <code>NA</code></li>
</ul></td>
</tr>
<tr class="even">
<td><code>STRSXP</code></td>
<td><code>StringSxp</code></td>
<td><code>OwnedStringSxp</code></td>
<td></td>
</tr>
<tr class="odd">
<td><code>VECSXP</code></td>
<td><code>ListSxp</code></td>
<td><code>OwnedListSxp</code></td>
<td></td>
</tr>
<tr class="even">
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
