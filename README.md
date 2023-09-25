
<!-- README.md is generated from README.qmd. Please edit that file -->

# Savvy - An unfriendly R extension interface using Rust

<!-- badges: start -->

[![](https://img.shields.io/github/actions/workflow/status/yutannihilation/savvy/R-CMD-check.yaml?style=flat-square&logo=github)](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml)
[![](https://img.shields.io/crates/v/savvy.svg?style=flat-square&logo=rust)](https://crates.io/crates/savvy)
[![](https://img.shields.io/docsrs/savvy.svg?style=flat-square&logo=docsdotrs)](https://docs.rs/savvy/latest/)

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
corresponding R function `to_upper()`. As you can see, this framework is
unfriendly in that this requires more explicit operations than extendr.
For the full details, please read [the crate
documentation](https://yutannihilation.github.io/savvy/savvy/index.html).

``` rust
/// Convert to Upper-case
/// 
/// @param x A character vector.
/// @export
#[savvy]
fn to_upper(x: StringSxp) -> savvy::Result<savvy::SEXP> {
    // Use `Owned{type}Sxp` to allocate an R vector for output.
    let mut out = OwnedStringSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        // To Rust, missing value is an ordinary value. In `&str`'s case, it's just "NA".
        // You have to use `.is_na()` method to distinguish the missing value.
        if e.is_na() {
            // Values need to be set by `set_elt()` one by one.
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str())?;
    }

    // `Owned{type}Sxp` type implements `From` trait for `SEXP`, so you can use `into()`.
    Ok(out.into())
}
```

## Getting Started

### Prerequisite

Before starting, install a helper R package for savvy.

``` r
install.packages(
    "savvy",
    repos = c("https://yutannihilation.r-universe.dev", "https://cloud.r-project.org")
)
```

Note that, under the hood, this is just a simple wrapper around
`savvy-cli`. So, if you prefer shell, you can directly use the CLI
instead, which is available on the
[releases](https://github.com/yutannihilation/savvy/releases).

### Create a new R package

First, create a new package. `usethis::create_package()` is convenient
for this.

``` r
usethis::create_package("path/to/foo")
```

Then, move to the package directory and generate necessary files like
`Makevars` and `Cargo.toml`, as well as generating C and R wrapper code
corresponding to the Rust code. `savvy::savvy_init()` does this all
(under the hood, this simply runs `savvy-cli init`).

Lastly, run `devtools::document()` to generate `NAMESPACE` and
documents.

``` r
savvy::savvy_init()
devtools::document()
```

Now, this package is ready to install!

### Update wrapper files

After modifying or adding some Rust code, you can update the C and R
wrapper files by running `savvy::savvy_update()` (under the hood, this
simply runs `savvy-cli update`).

``` r
savvy::savvy_update()
devtools::document()
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

But, static analysis has its pros and cons. While it would be good at
parsing a single file `lib.rs`, it’s probably hard to understand
multiple Rust files correctly. For example, if `misc.rs` defines a
function with `#[savvy]` and `lib.rs` imports it, it might be hard to
infer how the function is exported. On the other hand, extendr collects
the metadata on compile time. So, this sort of problems should never
happen.

#### Usage

``` console
Generate C bindings and R bindings for a Rust library

Usage: savvy-cli <COMMAND>

Commands:
  c-header      Generate C header file
  c-impl        Generate C implementation for init.c
  r-impl        Generate R wrapper functions
  makevars      Generate Makevars
  makevars-win  Generate Makevars.win
  gitignore     Generate .gitignore
  update        Update wrappers in an R package
  init          Init savvy-powered Rust crate in an R package
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
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
library(savvyExamples)

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
<col style="width: 19%" />
<col style="width: 24%" />
<col style="width: 24%" />
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
</tbody>
</table>
