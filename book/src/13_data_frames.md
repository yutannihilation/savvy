# Handling Data Frames

A `data.frame` is a list. You should simply handle it as a list in Rust code, and
all `data.frame`-related operations should be done in R code.

For example, if you want to return the result as a `data.frame`, the Rust
function should return a list, and wrapped by an R function that converts the
list into a data.frame. `tibble::as_tibble()` should be the right choice for
this purpose. Or, if you prefer lightweight dependency, you can use
`vctrs::new_data_frame()`, or simply `as.data.frame()`.

```rust
/// @export
#[savvy]
fn foo_impl() -> savvy::Result<savvy::Sexp> {
    // create a named list
    let mut out = savvy::OwnedListSexp::new(2, true)?;

    let x: Vec<f64> = some_function();
    let y: Vec<f64> = another_function();
    
    out.set_name_and_value(0, "x", OwnedRealSexp::try_from_slice(x)?)?;
    out.set_name_and_value(1, "y", OwnedRealSexp::try_from_slice(y)?)?;

    out.into()
}
```
```r
foo <- function() {
  result <- foo_impl()
  tibble::as_tibble(result)
}
```