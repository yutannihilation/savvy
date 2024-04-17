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
    specify the location by `--cache_dir`.

Note that, this takes the path to the root of **a crate**, not that of an R
package.

```sh
savvy-cli test path/to/your_crate
```

### Test module

You can write tests under a module marked with `#[cfg(test)]`. A `#[test]`
function needs to have the return value of `savvy::Result<()>`, which is the
same convention as `#[savvy]`.

```rust
#[cfg(test)]
mod test {
    use savvy::OwnedIntegerSexp;

    #[test]
    fn test_integer() -> savvy::Result<()> {
        let mut x = OwnedIntegerSexp::new(3)?;
        assert_eq!(x.as_slice(), &[0, 0, 0]);
        Ok(())
    }
}
```

To test a function that takes user-supplied SEXPs like `IntegerSexp`, you can
use `.as_read_only()` to convert from the corresponding `Owned-` type. For
example, if you have a function `your_fn()` that accepts `IntegerSexp`, you can
construct an `OwnedIntegerSexp` and convert it to `IntegerSexp` before passing
it to `your_fn()`.

```rust
fn your_fn(x: IntegerSexp) -> savvy::Result<()> {
    // ...snip...
}

#[cfg(test)]
mod test {
    use savvy::OwnedIntegerSexp;

    #[test]
    fn test_integer() -> savvy::Result<()> {
        let x = savvy::OwnedIntegerSexp::new(3)?;
        let x_ro = x.as_read_only();
        assert!(super::your_fn(x_ro).is_ok());
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

If you need some other crate for the test code, you can just use
`[dev-dependencies]` section of the `Cargo.toml` then `savvy-cli test` will pick
it.