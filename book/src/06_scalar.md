# Handling Salars

You can specify scalar types for input and output like below.

```rust
#[savvy]
fn scalar_input_int(x: i32) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}
```

```rust
#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    1.try_into()
}
```

## Missing values

If the type of the input is scalar, `NA` is always rejected. This is
inconsistent with the rule for vector input, but, this is my design decision in
the assumption that a scalar missing value is rarely found useful on Rust's
side.

```rust
#[savvy]
fn identity_logical_single(x: bool) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(1)?;
    out.set_elt(0, x)?;
    out.into()
}
```

```r
identity_logical_single(NA)
#> Error in identity_logical_single(NA) : 
#>   Must be length 1 of non-missing value
```
