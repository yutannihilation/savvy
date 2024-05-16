# Optional Argument

To represent an optional argument, you can wrap it with `Option`. Then, the
corresponding R function sets the default value of `NULL` on the argument.

``` rust
#[savvy]
fn default_value_vec(x: Option<IntegerSexp>) -> savvy::Result<Sexp> {
    if let Some(x) = x {
        x.iter().sum::<i32>().try_into()
    } else {
        (-1).try_into()
    }
}
```

``` r
function(x = NULL) {
  .Call(savvy_default_value_vec__impl, x)
}
```

This function works with or without the argument.

``` r
default_value_vec(1:10)
#> [1] 55

default_value_vec()
#> [1] -1
```
