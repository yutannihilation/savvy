# Introduction

**savvy** is a simple R extension interface using Rust, like the
[extendr](https://extendr.github.io/) framework. The name “savvy” comes
from the Japanese word “錆” (pronounced as `sàbí`), which means “Rust”.

With savvy, you can automatically generate R functions from Rust code.
This is an example of what savvy-powered function would look like:

**Rust**

``` rust
use savvy::savvy;
use savvy::NotAvailableValue;   // for is_na() and na()

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

**R**

``` r
to_upper(c("a", "b", "c"))
#> [1] "A" "B" "C"
```

## Examples

A toy example R package can be found in [`R-package/`
directory](https://github.com/yutannihilation/savvy/tree/master/R-package).

## Links

* [crates.io](https://crates.io/crates/savvy)
* [API reference](https://docs.rs/savvy/latest/)
* [API reference (dev version)](https://yutannihilation.github.io/savvy/reference/savvy/)