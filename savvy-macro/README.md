# savvy-macro

Generate R-ready Rust functions by adding `#[savvy]` macro.

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