# Handling Factors

A factor is internally an integer vector with the `levels` attribute. You can
handle this on Rust's side, but the recommended way is to write a wrapper R
function to convert the factor vector to a character vector.

Say there's a Rust function that takes a character vector as its argument.

```rust
#[extendr]
fn foo_impl(x: StringSexp) -> savvy::Result<()> {
    ...
}
```

Then, you can write a function like below to convert the input to a character
vector. If you want better validation, you can use `vctrs::vec_cast()` instead.

```r
foo <- function(x) {
    x <- as.character(x)
    foo_impl(x)
}
```

If you need the information of the order of the levels, you should pass it as an
another argument.

```rust
#[extendr]
fn foo_impl2(x: StringSexp, levels: StringSexp) -> savvy::Result<()> {
    ...
}
```

```r
foo2 <- function(x) {
    levels <- levels(x)
    x <- as.character(x)
    foo_impl2(x, levels)
}
```

