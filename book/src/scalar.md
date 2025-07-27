# Handling Scalar

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

| R type            | Rust scalar type         |
| :---------------- | :----------------------- |
| integer           | `i32`                    |
| double            | `f64`                    |
| logical           | `bool`                   |
| raw               | `u8`                     |
| character         | `&str`                   |
| complex           | `num_complex::Complex64` |
| integer or double | `savvy::NumericScalar`   |

### `NumericScalar`

`NumericScalar` is a special type that can handle both integeer and double. You
can get the value from it by `as_i32()` for `i32`, or `as_f64()` for `f64`.
These method converts the value if the input type is different from the target
type.

```rust
#[savvy]
fn times_two_numeric_i32_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    let v = x.as_i32()?;
    if v.is_na() {
        (i32::na()).try_into()
    } else {
        (v * 2).try_into()
    }
}
```

Note that, while `as_f64()` is infallible, `as_i32()` can fail when the
conversion is from `f64` to `i32` and

- the value is `Inf` or `-Inf`
- the value is out of range for `i32`
- the value is not integer-ish (e.g. `1.1`)

For convenience, `NumericScalar` also provides a conversion to usize by
`as_usize()`. What's good is that this can handle integer-ish numeric, which
means you can allow users to input a larger number than the integer max
(2147483647)!

```rust
fn usize_to_string_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    let x_usize = x.as_usize()?;
    x_usize.to_string().try_into()
}
```

```r
usize_to_string_scalar(2147483648)
#> [1] "2147483648"
```

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

If you want to accept `NA`, the primary recommendation is to handle it in R
code. But, you can also use `Sexp` as input. You can detect a missing value
by `is_scalar_na()` and then convert it to a specific type by `try_into()`.

```rust
/// @export
#[savvy]
fn times_two_numeric_i32_scalar_v2(x: savvy::Sexp) -> savvy::Result<savvy::Sexp> {
    if x.is_scalar_na() {
        return (i32::na()).try_into();
    }

    let x_num: NumericScalar = x.try_into()?;
    let v = x.as_i32()?;

    // Note: NA check is already done, so you don't need to check v.is_na()

    (v * 2).try_into()
}
```
