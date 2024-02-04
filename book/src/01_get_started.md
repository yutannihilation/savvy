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

Now, this package is ready to install! After installing (e.g. by running
"Install Package" on RStudio IDE), confirm you can run this example function
that multiplies the first argument by the second argument.

```r
library(<your package>)

int_times_int(1:4, 2L)
#> [1] 2 4 6 8
```

### Package structure

After `savvy::savvy_init()`, the structure of your R package should look like below.

```
.
├── DESCRIPTION
├── NAMESPACE
├── R
│   └── wrappers.R      <---(1)
├── configure           <---(2)
├── foofoofoofoo.Rproj
└── src
    ├── Makevars.in     <---(2)
    ├── Makevars.win    <---(2)
    ├── init.c          <---(3)
    └── rust
        ├── api.h       <---(3)
        ├── Cargo.toml  <---(4)
        └── src
            └── lib.rs  <---(4)
```

1. `wrappers.R`: R functions for the corresponding Rust functions
2. `configure`, `Makevars.in`, and `Makevars.win`: Necessary build settings for compiling Rust code
3. `init.c` and `api.h`: C functions for the corresponding Rust functions
4. `Cargo.toml` and `lib.rs`: Rust code

## Write your own function

The most revolutionary point of `savvy::savvy_init()` is that it leaves the most
important task to you; let's define a typical hello-world function for practice!

### Write some Rust code

Open `src/rust/lib.rs` and add the following lines. `r_println!` is the R
version of `println!` macro.

```rust
/// @export
#[savvy]
fn hello() -> savvy::Result<()> {
    savvy::r_println!("Hello world!");
    Ok(())
}
```

### Update wrapper files

Every time you modify or add some Rust code, you need to update the C and R
wrapper files by running `savvy::savvy_update()` (under the hood, this simply
runs `savvy-cli update`). Don't forget to run `devtools::document()` as well.

``` r
savvy::savvy_update()
devtools::document()
```

After re-installing your package, you should be able to run the `hello()`
function on your R session.

```r
hello()
#> Hello world!
```