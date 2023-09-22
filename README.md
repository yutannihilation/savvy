
<!-- README.md is generated from README.qmd. Please edit that file -->

# Savvy - An unfriendly R extension interface using Rust

<!-- badges: start -->

<div>

[![](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml)

</div>

<!-- badges: end -->

## What the hell is this?? Why do you need another framework when there’s extendr?

This is nothing but my personal challenge to re-invent the wheel in
order to get better understanding about what
[extendr](https://extendr.github.io/) does. While this is usable,
ergonomics is not included. If you prefer friendliness, please use
extendr.

### Why savvy?

In Japanese, “Rust” is pronounced as `sàbí`(錆). Since the sound is
similar, and this framework is intended to be used by R-API-savvy
people, I chose this name.

## Example Rust code

With savvy, you can implement Rust function like below to create the
corresponding R function `to_upper()`.

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

As you can see, this framework is unfriendly in that this requires more
explicit operations than extendr.

- All inputs are read-only. You always have to allocate a new SEXP for
  the return value.
- The function’s arguments must be savvy types (`...Sxp`). Scalar types
  (`i32`, `f64`, `bool`, and `&str`) are also allowed, though.
- The function’s return type must be either `savvy::Result<savvy::SEXP>`
  or `()`.
- Savvy doesn’t take care of the output conversion. You have to create a
  new SEXP object by `Owned...Sxp::new()` and set values by `set_elt()`
  one by one.

## Getting Started

### Prerequisite

Before starting, get `savvy-cli` command either by downloading the
binary from
[Releases](https://github.com/yutannihilation/savvy/releases) or by
`cargo install`:

``` sh
 cargo install --git https://github.com/yutannihilation/savvy savvy-cli
```

### Create a new R package

First, create a new package. `usethis::create_package()` is convenient
for this.

``` r
usethis::create_package("path/to/foo")
```

Next, run `savvy-cli init` against the package directory. This will
create necessary files like `Makevars` and `Cargo.toml`, as well as
generating C and R wrapper code corresponding to the Rust code.

``` sh
savvy-cli init path/to/foo
```

Lastly, open an R session in the package directory and run
`devtools::document()` to generate `NAMESPACE`.

``` r
devtools::document()
```

Now, this package is ready to install!

### Update wrapper files

After writing more Rust code, you can update the C and R wrapper files
by running `savvy-cli update`.

``` sh
savvy-cli update path/to/foo
```

To update the documents, you also have to run `devtools::document()`
again.

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

The binary can be found in the
[Releases](https://github.com/yutannihilation/savvy/releases) section.

#### Usage

``` console
Generate C bindings and R bindings for a Rust library

Usage: savvy-cli.exe <COMMAND>

Commands:
  c-header  Generate C header file
  c-impl    Generate C implementation for init.c
  r-impl    Generate R wrapper functions
  update    Update wrappers in an R package
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` sh
cargo install --path .\savvy-cli\
savvy-cli update .\R-package\
```

## Crates

- `savvy`: a simple wrapper around R’s C API
- `savvy-bindgen`: a crate for generating C and R bindings from Rust
  code
- `savvy-cli`: a CLI to invoke `savvy-bindgen`’s functionalities
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
<col style="width: 18%" />
<col style="width: 23%" />
<col style="width: 26%" />
<col style="width: 29%" />
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
