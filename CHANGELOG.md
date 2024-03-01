# Changelog

<!-- next-header -->
## [Unreleased] (ReleaseDate)

### Fixed bugs

* Reject invalid external pointers so that the R session doesn't crash.

## [v0.2.14] (2024-02-14)

### Fixed bugs

* `savvy-cli update` and `savvy-cli init` now correctly overwrite the existing
  files.

## [v0.2.13] (2024-02-14)

### Fixed bugs

* The savvy-cli crate now requires Rust >= 1.74 because this is clap's MSRV.

## [v0.2.12] (2024-02-14)

### New Features

* `savvy-cli init` now adds `SystemRequirements` to the `DESCRIPTION` file.

### Fixed bugs

* `savvy-cli` now works if it's invoked outside of the R package directory.
* `savvy-cli init` now generates the build configuration with a workaround for
  the case of the `gnu` toolchain on Windows.

## [v0.2.11] (2024-02-04)

## [v0.2.10] (2024-02-04)

### Fixed bugs

* Fix the wrong implementation of `to_vec()`.

## [v0.2.9] (2024-01-27)

### Breaking changes

* `Function.call()` now uses `FunctionArgs` to represent function arguments.
  This is necessary change in order to protect function arguments from GC-ed
  unexpectedly. The previous interface requires users to pass `Sexp`, which is
  unprotected.
* `Function.call()` now doesn't require the environment to be executed because
  it rarely matters. Accordingly, `Environment` is removed from the API.

## [v0.2.8] (2024-01-26)

### Breaking changes

* `savvy-cli init` now produces `Makevars.in` and `configure` instead of
  `Makevars` in order to support WebR transparently. One limitation on Windows
  is that `configure` cannot be set executable; you have to run the following
  command by yourself.

  ```sh
  git update-index --add --chmod=+x ./configure
  ```

### New features

* Add an experimental support for function and environment.

## [v0.2.7] (2024-01-25)

### New features

* (Experimentally) support WebR by not using `R_UnwindProtect()`.

## [v0.2.6] (2024-01-20)

### Fixed bugs

* Fix misuses of `Rf_mkCharLenCE()` which caused compilation error on ARM64
  Linux.

## [v0.2.5] (2024-01-20)

### Breaking changes

* `ListSexp` now returns an `Sexp` instead of a `TypedSexp`. Use `.into_typed()`
  to convert an `Sexp` to a `TypedSexp`.

### New features

* Add `is_null()`.
* Add `as_read_only()` to `OwnedListSexp` as well.
* Add `cast_unchecked()` and `cast_mut_unchecked()` for casting an external
  pointer to a concrete type. Note that this is only needed for "external"
  external pointers.

## [v0.2.4] (2024-01-15)

## [v0.2.2] (2024-01-15)

### Breaking changes

* `r_print!` and `r_eprint!` are now macro that wraps `format!`, so you can use
  them just like Rust's `print!` macro. There are also `r_println!` and
  `r_eprintln!` available.

### New features

* Support scalar `usize` input.
* Add methods to access and modify attributes:
  * `get_attrib()` / `set_attrib()`
  * `get_names()` / `set_names()`
  * `get_class()` / `set_class()`
  * `get_dim()` / `set_dim()`
* A struct marked with `#[savvy]` now has `try_from()` for `Sexp`.

### Fixed bugs

* Newly-created R vectors (`Owned*Sexp`) are now properly initialized. If you
  really want to skip the initialization for some great reason, you can use
  `new_without_init()` instead of `new()`.
* `#[savvy]` now accepts `savvy::Sexp` as input.

<!-- next-url -->
[Unreleased]: https://github.com/yutannihilation/savvy/compare/v0.2.14...HEAD
[v0.2.14]: https://github.com/yutannihilation/savvy/compare/v0.2.13...v0.2.14
[v0.2.13]: https://github.com/yutannihilation/savvy/compare/v0.2.12...v0.2.13
[v0.2.12]: https://github.com/yutannihilation/savvy/compare/v0.2.11...v0.2.12
[v0.2.11]: https://github.com/yutannihilation/savvy/compare/v0.2.10...v0.2.11
[v0.2.10]: https://github.com/yutannihilation/savvy/compare/v0.2.9...v0.2.10
[v0.2.9]: https://github.com/yutannihilation/savvy/compare/v0.2.8...v0.2.9
[v0.2.8]: https://github.com/yutannihilation/savvy/compare/v0.2.7...v0.2.8
[v0.2.7]: https://github.com/yutannihilation/savvy/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/yutannihilation/savvy/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/yutannihilation/savvy/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/yutannihilation/savvy/compare/savvy-v0.2.2...v0.2.4
[v0.2.2]: https://github.com/yutannihilation/savvy/compare/savvy-v0.2.1...savvy-v0.2.2
