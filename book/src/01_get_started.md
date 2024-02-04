# Getting Started

## Prerequisite

Before starting, install a helper R package for savvy.

``` r
install.packages(
    "savvy",
    repos = c("https://yutannihilation.r-universe.dev", "https://cloud.r-project.org")
)
```

Note that, under the hood, this is just a simple wrapper around `savvy-cli`. So,
if you prefer shell, you can directly use the CLI instead, which is available on
the [releases](https://github.com/yutannihilation/savvy/releases).

## Create a new R package

First, create a new R package. `usethis::create_package()` is convenient for
this.

``` r
usethis::create_package("path/to/foo")
```

Then, move to the package directory and generate necessary files like `Makevars`
and `Cargo.toml`, as well as the C and R wrapper code corresponding to the Rust
code. `savvy::savvy_init()` does this all (under the hood, this simply runs
`savvy-cli init`).

Lastly, run `devtools::document()` to generate `NAMESPACE` and documents.

``` r
savvy::savvy_init()
devtools::document()
```

Now, this package is ready to install!

## Update wrapper files

After modifying or adding some Rust code, you can update the C and R wrapper
files by running `savvy::savvy_update()` (under the hood, this simply runs
`savvy-cli update`).

``` r
savvy::savvy_update()
devtools::document()
```
