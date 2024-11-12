# Savvy - A simple R extension interface using Rust


<!-- README.md is generated from README.qmd. Please edit that file -->

<!-- badges: start -->

[![](https://img.shields.io/github/actions/workflow/status/yutannihilation/savvy/R-CMD-check.yaml?style=flat-square&logo=github)](https://github.com/yutannihilation/savvy/actions/workflows/R-CMD-check.yaml)
[![](https://img.shields.io/crates/v/savvy.svg?style=flat-square&logo=rust)](https://crates.io/crates/savvy)
[![](https://img.shields.io/docsrs/savvy.svg?style=flat-square&logo=docsdotrs)](https://docs.rs/savvy/latest/)
[![](https://img.shields.io/badge/%C2%AF%5C_(%E3%83%84)_%2F%C2%AF-green?style=flat-square&logo=docsdotrs&label=docs%20(dev)&labelColor=grey)](https://yutannihilation.github.io/savvy/reference/savvy/)

<!-- badges: end -->

**savvy** is a simple R extension interface using Rust, like the
[extendr](https://extendr.github.io/) framework. The name “savvy” comes
from the Japanese word “錆” (pronounced as `sàbí`), which means “Rust”.

With savvy, you can automatically generate R functions from Rust code.
This is an example of what a savvy-powered function would look like:

**Rust**

``` rust
use savvy::savvy;

/// Convert to Upper-case
/// 
/// @param x A character vector.
/// @export
#[savvy]
fn to_upper(x: StringSexp) -> savvy::Result<savvy::Sexp> {
    // Use `Owned{type}Sexp` to allocate an R vector for output.
    let mut out = OwnedStringSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        // To Rust, missing value is an ordinary value. In `&str`'s case, it's just "NA".
        // You have to use `.is_na()` method to distinguish the missing value.
        if e.is_na() {
            // Set the i-th element to NA
            out.set_na(i)?;
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str())?;
    }

    out.into()
}
```

**R**

``` r
to_upper(c("a", "b", "c"))
#> [1] "A" "B" "C"
```

## Documents

- [user guide](https://yutannihilation.github.io/savvy/guide/)
- [savvy 入門](https://yutani.quarto.pub/intro-to-savvy-ja/) (Japanese)

## Contributing

[CONTRIBUTING.md](./CONTRIBUTING.md)

## Examples

A toy example R package can be found in [`R-package/`
directory](https://github.com/yutannihilation/savvy/tree/main/R-package).

Savvy is used in the following R packages:

- [prqlr](https://prql.github.io/prqlc-r/)
- [neopolars](https://eitsupi.r-universe.dev/neopolars)
- [string2path](https://yutannihilation.github.io/string2path/)

## Thanks

Savvy is not quite unique. This project is made possible by heavily
taking inspiration from other great projects:

- The basic idea is of course based on
  [extendr](https://github.com/extendr/extendr/). Savvy would not exist
  without extendr.
- [cpp11](https://cpp11.r-lib.org/)’s “writable” concept influenced the
  design a lot. Also, I learned a lot from the great implementation such
  as [the protection
  mechanism](https://cpp11.r-lib.org/articles/internals.html#protection).
- [PyO3](https://github.com/PyO3/pyo3) made me realize that the FFI
  crate doesn’t need to be a “sys” crate.
