# Handling Vector Input

## Basic rule

As described in [Key Ideas](./02_key_ideas.md), the input SEXP is **read-only**.
You cannot modify the values in place.

## Methods

### 1. `iter()`

`IntegerSexp`, `RealSexp`, `LogicalSexp`, and `StringSexp` provide `iter()`
method so that you can access to the value one by one. This can be efficient
when the data is too large to copy.

```rust
for (i, e) in x.iter().enumerate() {
    // ...snip...
}
```

### 2. `as_slice()` (for integer and numeric)

`IntegerSexp` and `RealSexp` can expose their underlying C array as a Rust slice
by `as_slice()`.

```rust
#[savvy]
fn foo(x: IntegerSexp) -> savvy::Result<()> {
    some_function_takes_slice(x.as_slice());
    Ok(())
}
```

Note that, while this is samely efficient for non-ALTREP vectors, this might be
costly for ALTREP vectors because an ALTREP needs to be materialized first. For
example, if the input is `1:1e8`, `iter()` should be more efficient.

### 3. `to_vec()`

As the name indicates, `to_vec()` copies values to a Rust vector. Copying can be
costly for big data, but a vector is handy if you need to pass the data around
among Rust functions.

```rust
let mut v = x.to_vec();
some_function_takes_vec(v);
```

If a function requires a slice and the type is not integer or numeric, you have
no choice to use `to_vec()` to create a new vector and then convert it to a
slice.

```rust
let mut v = x.to_vec();
another_function_takes_slice(&v);
```

## Missing values

There's no concept of "missing value" on the corresponding types of `Rust`. So,
it looks a normal value to Rust's side.

The good news is that R uses the sentinel values to represent `NA`, so it's
possible to check if a value is `NA` to R in case the type is either `i32`,
`f64` or `&str`.

* `i32`: [The minimum value of `int`][na_int] is used for representing `NA`.
* `f64`: [A special value][na_real] is used for representing `NA`.
* `&str`: [A `CHARSXP` of string `"NA"`][na_string] is used for representing
  `NA`; this cannot be distinguished by comparing the content of the string, but
  we can compare the pointer address of the underlying C `char` array.

[na_int]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/arithmetic.c#L143
[na_real]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/arithmetic.c#L90-L98
[na_string]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/names.c#L1219

You can check if the value is `NA` by `is_na()`, and refer to the sentinel value
of `NA` by `<T>::na()`. If you care about missing values, you always have to
have an `if` branch for missing values like below.

```rust
#[savvy]
fn sum(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let mut sum: f64 = 0.0;
    for e in x.iter() {
        if !e.is_na() {
            sum += e;
        }
    }

    ...snip...
}
```

The bad news is that `bool` is not the case. `bool` doesn't have `is_na()` or
`na()`. `NA` is treated as `TRUE` without any errors. So, you have to make sure
the input doesn't contain any missing values **on R's side**. For example, this
function is not an identity function.

```rust
#[savvy]
fn identity_logical(x: LogicalSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e)?;
    }

    out.into()
}
```

```text
> identity_logical(c(TRUE, FALSE, NA))
[1]  TRUE FALSE  TRUE
```
