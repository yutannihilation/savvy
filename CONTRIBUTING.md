Contributing to savvy
=====================

(Still work in progress...)

## Testing

### savvy crate

As savvy framework requires a real R session to work, `cargo test` doesn't work.
Instead, please use `savvy-cli test`.

```console
savvy-cli test .

# if you want to use the dev version of savvy-cli
cargo r-test
```

This extracts test code and creates a
temporary R package on-the-fly to run these tests.

The binary of [`savvy-cli`] is found on the [GitHub Releases][release]. You can
also install it via `cargo install`.

[release]: https://github.com/yutannihilation/savvy/releases

Currently, it also requires

* `R` is on PATH
* [savvy R package][R-pkg] is installed

[R-pkg]: [`savvy-cli`][release]

#### R package for testing

`R-package/` directory contains the R package for testing.

### savvy-macro crate

savvy-macro uses [insta](https://insta.rs/) for snapshot testing. Please install
`cargo-insta` first. The installation guide can be found on [the official
Getting Started][insta-install].

[insta-install]: https://insta.rs/docs/quickstart/

If you create a new snapshot or modify an existing snapshot, you can review and
accept the changes with:

```console
cargo insta review
```

### savvy-bindgen crate

You can just run `cargo test`.

```console
cargo test --manifest-path=./savvy-bindgen/Cargo.toml
```