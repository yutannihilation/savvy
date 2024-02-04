# Handling Vector Output

Basically, there are two ways to prepare an output to the R session.

## 1. Create a new R object first and put values on it

An owned SEXP can be allocated by using `Owned{type}Sexp::new()`. `new()` takes
the length of the vector as the argument. If you need the same length of vector
as the input, you can pass the `len()` of the input `SEXP`.

`new()` returns `Result` because the memory allocation can fail in case when the
vector is too large. You can probably just add `?` to it to handle the error
because a function marked with `#[savvy]` must return `Result`.

```rust
let mut out = OwnedStringSexp::new(x.len())?;
```

Use `set_elt()` to put the values one by one. Note that you can also assing
values like `out[i] = value` for integer and numeric. See [Type-specific
Topics](./07_type_specific.md) for more details.

```rust
for (i, e) in x.iter().enumerate() {
    // ...snip...

    out.set_elt(i, &format!("{e}_{y}"))?;
}
```

Then, you can convert it to `Result<Sexp>` by `into()`.

```rust
out.into()
```

## 2. Convert a Rust scalar or vector by `try_into()` at last

Another way is to use a Rust vector to store the results and convert it to an R
object at the end the function. This is fallible because this anyway needs to
create a new R object under the hood, which can fail. So, this time, the
conversion is done by `try_into()`, not by `into()`.

```rust
// Let's not consider for handling NAs at all for simplicity...

// vector output
#[savvy]
fn times_two(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let out: Vec<i32> = x.iter().map(|v| v * 2).collect();
    out.try_into()
}

// scalar output
#[savvy]
fn sum_real(x: RealSexp) -> savvy::Result<savvy::Sexp> {
    let sum: f64 = x.iter().sum();
    sum.try_into()
}
```

Note that, while this looks handy, this might not be very efficient; for example,
`times_two()` above allocates a Rust vector, and then copy the values into a new
R vector in `try_into()`. The copying cost can be innegligible when the vector
is very huge.
