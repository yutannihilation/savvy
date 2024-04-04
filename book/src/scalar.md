# Handling Salars

## Input

Scalar inputs are handled transparently. The corresponding types are shown in
the table below.

```rust
/// @export
#[savvy]
fn scalar_input_int(x: i32) -> savvy::Result<()> {
    savvy::r_println!("{x}");
    Ok(())
}
```

| R type     | Rust scalar type   |
|:-----------|:-------------------|
| integer    | `i32`              |
| numeric    | `f64`              |
| logical    | `bool`             |
| character  | `&str`             |
| complex    | `num_complex::Complex64` |

## Output

Just like a Rust vector, a Rust scalar value can be converted into `Sexp` by
`try_from()`. It's as simple as.

```rust
/// @export
#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    1.try_into()
}
```

Alternatively, the same conversion is available in the form of
`Owned{type}Sexp::try_from_scalar()`.

```rust
/// @export
#[savvy]
fn scalar_output_int() -> savvy::Result<savvy::Sexp> {
    let out = OwnedIntegerSexp::try_from_scalar(1)?;
    out.into()
}
```

## Missing values

If the type of the input is scalar, `NA` is always rejected. This is
inconsistent with the rule for vector input, but, this is my design decision in
the assumption that a scalar missing value is rarely found useful on Rust's
side.

```rust
/// @export
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
