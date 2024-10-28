Contributing to savvy
=====================

(Still work in progress...)

## Testing

### savvy crate

As savvy framework requires a real R session to work, `cargo test` doesn't work.
Instead, please use `savvy-cli test`. This extracts test code and creates a
temporary R package on-the-fly to run these tests.

```sh
savvy-cli test .
```

if you want to use the dev version of savvy-cli, you can run `cargo r-test`,
which is an alias of `cargo run --manifest-path ./savvy-cli/Cargo.toml -- test`.

The binary of `savvy-cli` is available from the [GitHub Releases][release]. You
can also install it via `cargo install`.

[release]: https://github.com/yutannihilation/savvy/releases

Currently, it also requires

* `R` is on PATH
* [pkgbuild R package][pkgbuild] is installed

[pkgbuild]: https://pkgbuild.r-lib.org/

#### R package for testing

`R-package/` directory contains the R package for testing. You can run
`devtools::check()` on the directory.

### savvy-macro crate

savvy-macro uses [insta](https://insta.rs/) for snapshot testing. Please install
`cargo-insta` first. The installation guide can be found on [the official
Getting Started][insta-install].

[insta-install]: https://insta.rs/docs/quickstart/

If you create a new snapshot or modify an existing snapshot, you can review and
accept the changes with:

```sh
cargo insta review
```

### savvy-bindgen crate

You can just run `cargo test`.

```sh
cargo test --manifest-path=./savvy-bindgen/Cargo.toml
```
