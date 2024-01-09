# savvy

Savvy is a simple R extension interface using Rust.

This is nothing but my personal challenge to re-invent the wheel in order to get
better understanding about what [extendr](https://extendr.github.io/) does.
While this is usable, ergonomics is not included. If you prefer friendliness,
please use extendr.

For the full details, please read [savvy's crate
documentation](https://docs.rs/savvy/latest/).

``` rust
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
            // Values need to be set by `set_elt()` one by one.
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str())?;
    }

    out.into()
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