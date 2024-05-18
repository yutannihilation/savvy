# Testing

## Write integration tests on R's side

The most recommended way is to write tests on R's side just as you do with an
ordinary R package. You can write tests on Rust's side as described later, but,
ultimately, the R functions are the user interface, so you should test the
behavior of actual R functions.

## Write Rust tests

The sad news is that `cargo test` doesn't work with savvy. This is because savvy
always requires a real R session to work. But, don't worry, `savvy-cli test` is
the tool for this. `savvy-cli test` does

1. extract the Rust code of the test modules and the doc tests
2. create a temporary R package[^1] and inject the extracted Rust code
3. build and run the test functions via the R package

[^1]: The R package is created in the OS's cache dir by default, but you can
    specify the location by `--cache-dir`.

Note that, this takes the path to the root of **a crate**, not that of an R
package.

```sh
savvy-cli test path/to/your_crate
```

### Limitations

`savvy-cli test` tries to mimic what `cargo test` does as much as possible, but
there's some limitations.

First, in order to run tests, you need to add `"lib"` to the `crate-type`. This
is because your crate is used as a Rust library when run by `savvy-cli test`.

```toml
[lib]
crate-type = ["staticlib", "lib"]
                           ^^^^^
```

Second, if you want to test a function or a struct, it must be public. For the
ones marked with `#[savvy]` are automatically made public, but, if you want to
test other functions, you need to add `pub` to it by yourself.

```rs
pub fn foo() -> savvy::Result<()> {
^^^
```

### Test module

You can write tests under a module marked with `#[cfg(feature = "savvy-test")]` instead of
`#[cfg(test)]`. A `#[test]` function needs to have the return value of
`savvy::Result<()>`, which is the same convention as `#[savvy]`.
To check if an SEXP contains the expected data, `assert_eq_r_code` is convenient.

```rust
#[cfg(feature = "savvy-test")]
mod test {
    use savvy::{OwnedIntegerSexp, assert_eq_r_code};

    #[test]
    fn test_integer() -> savvy::Result<()> {
        let mut x = OwnedIntegerSexp::new(3)?;

        assert_eq_r_code(x, "c(0L, 0L, 0L)");

        Ok(())
    }
}
```

Note that `savvy-test` is just a marker for `savvy-cli`, not a real feature. So,
in theory, you don't really need this. However, in reality, you probably want to
add it to the `[features]` section of `Cargo.toml` because otherwise Cargo warns.

```toml
[features]
savvy-test = []
```

To test a function that takes user-supplied SEXPs like `IntegerSexp`, you can
use `.as_read_only()` to convert from the corresponding `Owned-` type. For
example, if you have a function `your_fn()` that accepts `IntegerSexp`, you can
construct an `OwnedIntegerSexp` and convert it to `IntegerSexp` before passing
it to `your_fn()`.

```rust
#[savvy]
pub fn your_fn(x: IntegerSexp) -> savvy::Result<()> {
    // ...snip...
}

#[cfg(feature = "savvy-test")]
mod test {
    use savvy::OwnedIntegerSexp;

    #[test]
    fn test_integer() -> savvy::Result<()> {
        let x = savvy::OwnedIntegerSexp::new(3)?;
        let x_ro = x.as_read_only();
        let result = super::your_fn(x_ro);

        assert_eq_r_code(result, "...");
        
        Ok(())
    }
}
```

### Doc tests

You can also write doc tests. `savvy-cli test` wraps it with a function with the
return value of `savvy::Result<()>`, you can use `?` to extract the `Result`
value in the code.

```rust
/// ```
/// let x = savvy::OwnedIntegerSexp::new(3)?;
/// assert_eq!(x.as_slice(), &[0, 0, 0]);
/// ```
```

### Features and dependencies

If you need to specify some features for testing, use `--features` argument.

```sh
savvy-cli test --features foo path/to/your_crate
```

For dependencies, `savvy-cli test` picks all dependencies in `[dependencies]`
and `[dev-dependencies]`. If you need some additional crate for the test code,
you can just use `[dev-dependencies]` section of the `Cargo.toml` just as you do
when you do `cargo test`.


### Reminder: You can use `cargo test`

While `#[savvy]` requires a real session, you can utilize `cargo test` by
separating the actual logic to a function that doesn't rely on savvy. For
example, suppose you have the following function `times_two_int()` that doubles
the input numbers.

```rust
#[savvy]
fn times_two_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
        } else {
            out[i] = e * 2;
        }
    }

    out.into()
}
```

In this case, you can rewrite the code to the following so that you can test
`times_two_int_impl()` with `cargo test`.

```rust
#[savvy]
fn times_two_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let result: Vec<i32> = times_two_int_impl(x.as_slice());
    result.try_into()
}

fn times_two_int_impl(x: &[i32]) -> Vec<i32> {
    x.iter()
        .map(|x| if x.is_na() { *x } else { *x * 2 })
        .collect::<Vec<i32>>()
}
```

But, as you might notice, this implementation is a bit inefficient that it
allocates a `Vec<i32>` just to store the temporary result. Like this, separating
a function might be a bit tricky and it might not be really worth in some cases.
(In this case, probably the function can return an iterator).