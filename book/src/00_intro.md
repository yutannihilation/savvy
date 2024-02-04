# Introduction

**savvy** is a simple R extension interface using Rust, like the [extendr]
framework.

With savvy, you can automatically generate R functions from Rust code. This is
an example of what savvy-powered function would look like.

**Rust**:

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

**R**:

``` r
to_upper(c("a", "b", "c"))
#> [1] "A" "B" "C"
```


[extendr]: https://extendr.github.io/
