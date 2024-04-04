# Changelog

<!-- next-header -->
## [Unreleased] (ReleaseDate)

### New features

* `OwnedListSexp` and `ListSexp` gains `unchecked_*()` variants of the `set` and
  `get` methods for a fast but unsafe operation. Thanks @daniellga!

## [v0.4.2] (2024-04-01)

### New features

* `OwnedIntegerSexp` and etc now have `try_from_iter()` method for constructing
  a new instance from an iterator.

  Example:

  ```rust
  #[savvy]
  fn filter_integer_odd(x: IntegerSexp) -> savvy::Result<Sexp> {
      // is_na() is to propagate NAs
      let iter = x.iter().copied().filter(|i| i.is_na() || *i % 2 == 0);
      let out = OwnedIntegerSexp::try_from_iter(iter)?;
      out.into()
  }
  ```

* `OwnedIntegerSexp` and etc now have `try_from_slice()` method for constructing
  a new instance from a slice or vec. This conversion is and has been possible
  via `try_from()`, but this method was added for discoverability.

* `OwnedIntegerSexp` and etc now have `try_from_scalar()` method for
  constructing a new instance from a scalar value (e.g. `i32`). This conversion
  is and has been possible via `try_from()`, but this method was added for
  discoverability.

* `savvy-cli update` and `savvy-cli init` now tries to parse the Rust files
  actually declared by `mod` keyword.

## [v0.4.1] (2024-03-30)

### Breaking changes

* `Sexp` loses `is_environment()` method becuase this isn't useful, considering
  savvy doesn't support environment.

### New features

* `get_dim()` and `set_dim()` are now available also on `Sexp`.

* Now savvy allows to consume the value behind an external pointer. i.e., `T`
  instead of `&T` or `&mut T` as the argument. After getting consumed, the
  pointer is null, so any function call on the already-consumed R object results
  in an error. See [the guide](https://yutannihilation.github.io/savvy/guide/10_struct.html) for more details.
  
  Example:

  ```rust
  struct Value {};
  struct Wrapper { inner: Value }

  #[savvy]
  impl Value {
    fn new() -> Self {
      Self {}
    }
  }

  #[savvy]
  impl Wrapper {
    fn new(value: Value) -> Self {
      Self { inner: value }
    }
  }
  ```

  ```r
  v <- Value$new()
  w <- Wrapper$new(v)  # value is consumed here.

  w <- Wrapper$new(v)
  #> Error: This external pointer is already consumed or deleted
  ```

* `Sexp` now has `assert_integer()` etc to verify the type of the underlying
  SEXP is as expected.

## [v0.4.0] (2024-03-27)

### Breaking changes

* `#[savvy]` on a struct's `impl` now generates the same name of R object that
  holds all the accociated functions. For example, previously the below code
  generates a constructor `Person()`, but now the constructor is available as
  `Person$new()`.

  ```rust
  struct Person {
      pub name: String,
  }
  
  /// @export
  #[savvy]
  impl Person {
      fn new() -> Self {
          Self {
              name: "".to_string(),
          }
      }
  }
  ```

### New features

* A struct marked with `#[savvy]` can be used as the return type of the
  associated function. In conjunction with the change in v0.3.0, now a
  user-defined struct can be used more flexibly than before. Please refer to
  [the "Struct" section of the
  guide](https://yutannihilation.github.io/savvy/guide/10_struct.html)
* An experimental support on complex is added under `compex` feature flag.
  `ComplexSexp` and `OwnedComplexSexp` are the corresponding Rust types.
* `OwnedIntegerSexp` and etc now have `set_na(i)` method for shorthand of
  `set_elt(i, T::na())`. This is particularly useful for `OwnedLogicalSexp`
  because its setter interface `set_elt()` only accepts `bool` and no missing
  values.

### Fixed bugs

* An expert-only method `new_without_init()` now skips initialization as
  intended.

## [v0.3.0] (2024-03-24)

### New features

* Now user-defined struct can be used as an argument of `#[savvy]`-ed functions.
  It must be specified as `&Ty` or `&mut Ty`, not `Ty`. 

  Example:
  
  ```rust
  struct Person {
      pub name: String,
  }
  
  #[savvy]
  impl Person {
      fn get_name(&self) -> savvy::Result<savvy::Sexp> {
          let name = self.name.as_str();
          name.try_into()
      }
  }
  
  #[savvy]
  fn get_name_external(x: &Person) -> savvy::Result<savvy::Sexp> {
      x.get_name()
  }
  ```

### Fixed bugs

* Previously, `savvy-cli init` and `savvy-cli update` didn't handle the package
  name properly ("packageName" vs "package_name"). Now it's fixed.

### Breaking changes

* While this is described in the New features section, it was already allowed to
  specify user-defined structs as argument if the user defines the necessary
  `TryFrom` implementations propoerly. At that time, specifying it without `&`
  was possible, but now it's not allowed. Anyway, as this was undocumented and
  expert-only usage, I expect no one notices this breaking change.

## [v0.2.20] (2024-03-23)

## [v0.2.19] (2024-03-23)

### New features

* `LogicalSexp` and `OwnedLogicalSexp` now have `as_slice_raw()` method.  This
    is an expert-only function which might be found useful when you really need
    to distinguish NAs.

### Minor improvements

* `savvy-cli init` now generates `<dllname>-win.def` to avoid the infamous
  "export ordinal too large" error on Windows.

## [v0.2.18] (2024-03-11)

### Minor improvements

* The version requirement is a bit more strict now.

## [v0.2.17] (2024-03-10)

### Breaking changes

* `get_dim()` now returns `&[i32]` instead of `Vec<usize>` to avoid allocation.
  If the matrix library requires `usize`, you need to convert the `i32` to
  `usize` by yourself now.
  Accordingly, `set_dim()` now accepts both `&[i32]` and `&[usize]`.

## [v0.2.16] (2024-03-03)

### Breaking changes

* `fake-libR` feature is withdrawn. Instead, Windows users can add this build
  configuration to avoid the linker error:
  ```toml
  [target.x86_64-pc-windows-msvc]
  rustflags = ["-C", "link-arg=/FORCE:UNRESOLVED"]
  ```

## [v0.2.15] (2024-03-02)

### New features

* Previously, if a crate uses savvy, `cargo test` fails to compile on Windows
  even if the test code doesn't use the savvy API at all. This is because the
  symbols from Rinternals.h needs to be resolved. You can add `savvy` with
  `fake-libR` feature in `dev-dependencies` to avoid this issue.

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

### New features

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
[Unreleased]: https://github.com/yutannihilation/savvy/compare/v0.4.2...HEAD
[v0.4.2]: https://github.com/yutannihilation/savvy/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/yutannihilation/savvy/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/yutannihilation/savvy/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/yutannihilation/savvy/compare/v0.2.20...v0.3.0
[v0.2.20]: https://github.com/yutannihilation/savvy/compare/v0.2.19...v0.2.20
[v0.2.19]: https://github.com/yutannihilation/savvy/compare/v0.2.18...v0.2.19
[v0.2.18]: https://github.com/yutannihilation/savvy/compare/v0.2.17...v0.2.18
[v0.2.17]: https://github.com/yutannihilation/savvy/compare/v0.2.16...v0.2.17
[v0.2.16]: https://github.com/yutannihilation/savvy/compare/v0.2.15...v0.2.16
[v0.2.15]: https://github.com/yutannihilation/savvy/compare/v0.2.14...v0.2.15
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
