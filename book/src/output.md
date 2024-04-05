# Handling Vector Output

Basically, there are two ways to prepare an output to the R session.

## 1. Create a new R object first and put values on it

An owned SEXP can be allocated by using `Owned{type}Sexp::new()`. `new()` takes
the length of the vector as the argument. If you need the same length of vector
as the input, you can pass the `len()` of the input `SEXP`.

`new()` returns `Result` because the memory allocation can fail in case when the
vector is too large. You can probably just add `?` to it to handle the error.

```rust
let mut out = OwnedStringSexp::new(x.len())?;
```

Use `set_elt()` to put the values one by one. Note that you can also assign
values like `out[i] = value` for integer and numeric. See [Type-specific
Topics](./07_type_specific.md) for more details.

```rust
for (i, e) in x.iter().enumerate() {
    // ...snip...

    out.set_elt(i, &format!("{e}_{y}"))?;
}
```

You can use `set_na()` to set the specified element as NA. For example, it's a
common case to use this in order to propagate the missingness like below.

```rust
for (i, e) in x.iter().enumerate() {
    // ...snip...
    if e.is_na() {
        out.set_na(i)?;
    } else {
        // ...snip...
    }
}
```

After putting the values to the vector, you can convert it to `Result<Sexp>` by
`into()`.

```rust
/// @export
#[savvy]
fn foo() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;

    // ...snip...

    out.into()
}
```

## 2. Convert a Rust vector by methods like `try_into()`

Another way is to use a Rust vector to store the results and convert it to an R
object at the end of the function. This is also fallible because this anyway
needs to create a new R object under the hood, which can fail. So, this time,
the conversion is `try_into()`, not `into()`.

```rust
// Let's not consider for handling NAs at all for simplicity...

/// @export
#[savvy]
fn times_two(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let out: Vec<i32> = x.iter().map(|v| v * 2).collect();
    out.try_into()
}
```

Note that, while this looks handy, this might not be very efficient; for example,
`times_two()` above allocates a Rust vector, and then copy the values into a new
R vector in `try_into()`. The copying cost can be innegligible when the vector
is very huge.


### `try_from_slice()`m

The same conversions are also available in the form of
`Owned{type}Sexp::try_from_slice()`. While this says "slice", this accepts
`AsRef<[T]>`, which means both `Vec<T>` and `&[T]` can be used.

For converting the return value, probably `try_from()` is shorter in most of the
cases. But, sometimes you might find this useful (e.g., the return value is a
list and you need to construct the elements of it).

```rust
/// @export
#[savvy]
fn times_two2(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let out: Vec<i32> = x.iter().map(|v| v * 2).collect();
    let out_sexp: OwnedIntegerSexp::try_from_slice(out);
    out_sexp.into()
}
```

### `try_from_iter()`

If you only have an iterator, `try_from_iter()` is more efficient. This example
function is the case. The previous examples first `collect()`ed into a `Vec`,
but it's not necessary in theory.

```rust
/// @export
#[savvy]
fn times_two3(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let iter = x.iter().map(|v| v * 2);
    let out_sexp: OwnedIntegerSexp::try_from_iter(iter);
    out_sexp.into()
}
```

Note that, if you already have a slice or vec, you should use `try_from_slice()`
instead of calling `iter()` on the slice or vec and using `try_from_iter()`. In
such cases, `try_from_slice()` is more performant for integer, numeric, and
complex because it just copies the underlying memory into SEXP rather than
handling the elements one by one.
