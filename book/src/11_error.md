# Error handling

To propagate your errors to the R session, you can return a `savvy::Error`. You
can easily create it by using `.into()` on a string of the error message.

```rust
#[savvy]
fn raise_error() -> savvy::Result<savvy::Sexp> {
    Err("This is my custom error".into())
}
```

```r
raise_error()
#> Error: This is my custom error
```

For the implementation details of the internals, please refer to [my blog
post](https://yutani.rbind.io/post/dont-panic-we-can-unwind/#implementation).
