# Changelog

<!-- next-header -->
## [Unreleased] (ReleaseDate)

### Breaking Change

Removed `TryFrom<Sexp> for usize`, so the following code no longer compiles.

```rust
#[savvy]
fn foo(x: usize) -> savvy::Result<()> {
    ...
}
```

Instead, you can use `i32` and convert it to `usize` by yourself. If you are
sure the input number is never negative, you can just use the `as` conversion.
If you are not sure, you should use `<usize>::try_from()` and handle the error
by yourself. Also, please be aware you need to handle NA as well.

```rust
#[savvy]
fn foo(x: i32) -> savvy::Result<()> {
    if x.is_na() {
        return Err("cannot convert NA to usize".into())?;
    }
    
    let x = <usize>::try_from(x).map_err(|e| e.to_string().into());

    ...
}
```

Alternatively, you can use newly-added methods, `NumericScalar::as_usize()` and
`NumericSexp::iter_usize()`. What's good is that this can handle integer-ish
numeric, which means you can allow users to input a larger number than the
integer max (2147483647)!

```rust
fn usize_to_string_scalar(x: NumericScalar) -> savvy::Result<Sexp> {
    let x_usize = x.as_usize()?;
    x_usize.to_string().try_into()
}
```

```r
usize_to_string_scalar(2147483648)
#> [1] "2147483648"
```

## [v0.6.8] (2024-09-17)

### Minor Improvements

* `savvy init` now generates
  * slightly better `configure` script that checks if `cargo` command is available
  * `cleanup` script to remove the generated `Makevars` after compilation
  * `configure.win` and `cleanup.win`
  * `src/Makevars.win.in` instead of `src/Makevars.win` for consistency with Unix-alikes

## [v0.6.7] (2024-09-05)

### Minor Improvements

* Remove the use of non-API call `Rf_findVarInFrame`.

* Now the GitHub release includes an installation script to install the savvy
  CLI into `$CARGO_HOME/bin`, thanks to [cargo-dist]. This should make it easier
  to use `savvy-cli` on CI. 
  ```sh
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/yutannihilation/savvy/releases/download/v0.6.7/savvy-cli-installer.sh | sh
  ```

[cargo-dist]: https://opensource.axo.dev/cargo-dist/

* Improve handling of raw identifiers (e.g. `r#struct`) more.

## [v0.6.6] (2024-09-04)

### Bug fixes

* Fix inappropriate use of `PROTECT`. Thanks @t-kalinowski!

* Handle raw identifiers (e.g. `r#struct`) correctly.

## [v0.6.5] (2024-07-02)

### New features

* Add support for raw, including ALTRAW.  
  Please be aware that, while this support was added for consistency, I bet it's
  really rare that a raw vector is actually needed; if you want to deal with a
  binary data on Rust's side, your primary option should be to store it in an
  external pointer (of a struct you define) rather than an R's raw vector.

### Minor Improvements

* Wrapper environment of a Rust struct or enum now cannot be modified by users.

* Remove the use of the following non-API calls:
  * `Rf_findVarInFrame3`
  * `STRING_PTR`
  * `DATAPTR`

## [v0.6.4] (2024-05-25)

### New features

* New function `r_warn()` safely show a warning. Note that, a warning can raise
  error when `options(warn = 2)`, so you should not ignore the error from
  `r_warn()`. The error should be propagated to the R session.

* Savvy now translates `Option<T>` as an optional argument, i.e., an argument
  with the default value of `NULL`.

  Example:
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
  default_value_vec(1:10)
  #> [1] 55

  default_value_vec()
  #> [1] -1
  ```

### Bug fixes

* `r_print!()` and `r_eprint!()` now can print strings containing `%`.

### Breaking Change

* The notation for `savvy-cli test` is now changed to `#[cfg(feature =
  "savvy-test")]` from `#[cfg(savvy_test)]`. This is to avoid the upcoming
  change in Cargo ([ref](https://blog.rust-lang.org/2024/05/06/check-cfg.html)).

## [v0.6.3] (2024-05-05)

### New features

* New types `NumericSexp` and `NumericScalar` are added to handle both integer
  and double. You can get a slice via `as_slice_*()` or an iterator via
  `iter_*()`.  

  ```rust
  #[savvy]
  fn times_two(x: NumericSexp) -> savvy::Result<Sexp> {
      let mut out = OwnedIntegerSexp::new(x.len())?;

      for (i, v) in x.iter_i32().enumerate() {
          let v = v?; // The item of iter_i32() is Result because the conversion can fail.
          if v.is_na() {
              out[i] = i32::na();
          } else {
              out[i] = v * 2;
          }
      }

      out.into()
  }
  ```

  You can also use `.into_typed()` to handle integer and double differently.

  ```rust
  #[savvy]
  fn times_two(x: NumericSexp) -> savvy::Result<savvy::Sexp> {
      match x.into_typed() {
          NumericTypedSexp::Integer(i) => times_two_int(i),
          NumericTypedSexp::Real(r) => times_two_real(r),
      }
  }
  ```

* Savvy now provides `r_stdout()` and `r_stderr()` to be used with interfaces
  that require `std::io::Write`. Also, you can use `savvy::log::env_logger()` to
  output logs to R's stderr. Here's an example usage:

  ```rust
  use savvy::savvy_init;
  use savvy_ffi::DllInfo;

  #[savvy_init]
  fn init_logger(dll_info: *mut DllInfo) -> savvy::Result<()> {
      savvy::log::env_logger().init();
      Ok(())
  }
  ```

### Breaking changes

* `AltList` now loses `names` argument in `into_altrep()` for consistency.
  Please use `set_names()` on the resulted `Sexp` object.

  ``` rust
  let mut out = v.into_altrep()?;
  out.set_names(["one", "two"])?;
  Ok(out)
  ```

## [v0.6.2] (2024-05-04)

### New features

* New macro `#[savvy_init]` makes the function executed when the DLL is loaded
  by R. This is useful for initializaing resources. See [the guide](https://yutannihilation.github.io/savvy/guide/initialization_routine.html) for more details.
  
  Example:
  ```rust
  use std::sync::OnceLock;

  static GLOBAL_FOO: OnceLock<Foo> = OnceLock::new();

  #[savvy_init]
  fn init_global_foo(dll_info: *mut DllInfo) -> savvy::Result<()> {
      GLOBAL_FOO.get_or_init(|| Foo::new());

      Ok(())
  }
  ```

* Savvy now experimentally supports ALTREP. See [the guide](https://yutannihilation.github.io/savvy/guide/altrep.html) for more details.
  
  Example:
  ```rust
  struct MyAltInt(Vec<i32>);

  impl MyAltInt {
      fn new(x: Vec<i32>) -> Self {
          Self(x)
      }
  }

  impl savvy::IntoExtPtrSexp for MyAltInt {}

  impl AltInteger for MyAltInt {
      const CLASS_NAME: &'static str = "MyAltInt";
      const PACKAGE_NAME: &'static str = "TestPackage";

      fn length(&mut self) -> usize {
          self.0.len()
      }

      fn elt(&mut self, i: usize) -> i32 {
          self.0[i]
      }
  }
  ```

## [v0.6.1] (2024-04-26)

### Minor improvements

* Now savvy no longer uses `SETLENGTH`, which is a so-called "non-API" thing.

## [v0.6.0] (2024-04-20)

### Breaking changes

* `savvy-cli test` now parses test modules marked with `#[cfg(savvy_test)]`
  instead of `#[cfg(test)]`. The purpose of this change is to let `cargo test`
  run for the tests unrelated to a real R sessions.

* Savvy now generates different names of Rust functions and C functions;
  previously, the original function name is used for the FFI functions, but now
  it's `savvy_{original}_ffi`. This change shouldn't affect ordinary users.
  
  This change was necessary to let `#[savvy]` preserve the original function so
  that we can write unit tests on the function easily. One modification is that
  the function is made public. For more details, please read the [Testing section](https://yutannihilation.github.io/savvy/guide/test.html)
  in the guide.

* The generated R wrapper file is now named as `000-wrappers.R` instead of
  `wrappers.R`. This makes the file is loaded first so that you can override
  some of the R functions (e.g., a `print()` method for an enum) in another R
  file. The old wrapper file `wrappers.R` is automatically deleted by `savvy-cli
  update`

### New features

* Added a function `eval_parse_text()`, which is an equivalent to R's idiom
  `eval(parse(text = ))`. This is mainly for testing purposes.

* Added a function `is_r_identical()`, which is an equivalent to R's
  `identical()`. This is mainly for testing purposes.

* Added a function `assert_eq_r_code()` if the first argument has the same data
  as the result of the R code of the second argument.

  Example:

  ```rust
  let mut x = savvy::OwnedRealSexp::new(3)?;
  x[1] = 1.0;
  x[2] = 2.0;
  assert_eq_r_code(x, "c(0.0, 1.0, 2.0)");
  ```

* `savvy-cli test` now picks `[dev-dependencies]` from the crate's `Cargo.toml`
  as the dependencies to be used for testing.

* `savvy-cli test` got `--features` argument to add features to be used for
  testing.

## [v0.5.3] (2024-04-16)

### New features

* Savvy now catches crash not only on the debug build, but also on the release
  build if `panic = "unwind"`. Instead, now `savvy-cli init` generates a
  `Cargo.toml` with a release profile of `panic = "abort"`. You need to modify
  this setting if you really want to catch panics on the release build.

* `savvy-cli update` now ensures `.Rbuildignore` contains `^src/rust/.cargo$`
  and `^src/rust/target$`.

* `savvy-cli test` now uses OS's cache dir instead of the `.savvy` directory.

### Fixed bugs

* Now `savvy-cli test` works for other crates than savvy.

## [v0.5.2] (2024-04-14)

### New features

* Now savvy's debug build (when `DEBUG` envvar is set to `true`, i.e.,
  `devtools::load_all()`), panic doesn't crash R session and shows bactrace.
  This is useful for investigating what's the cause of the panic.

  Please keep in mind that, in Rust, panic is an **unrecoverable error**. So,
  not crashing doesn't mean you are saved.

* `savvy-cli test` no longer relies on the savvy R package.

### Fixed bugs

* Fixed a bug in `try_from_iter()` when the actual length is different than the
  size reported by `size_hint()`.

* `savvy-cli test` now uses the local crate as the path dependency, instead of
  using the savvy crate fixedly.

## [v0.5.1] (2024-04-13)

### New features

* An experimental new subcommand `savvy-cli test` runs tests by extracting and
  wrapping the test code with a temporary R package. This is because savvy
  always requires a real R session, which means `cargo test` doesn't work. Note
  that this relies on the savvy R package. Please install it before trying this.
  ```r
  install.packages("savvy", repos = c("https://yutannihilation.r-universe.dev", "https://cloud.r-project.org"))
  ```

* `savvy-cli init` now generates `Makevars` that supports debug build when
  `DEBUG` envvar is set to `true` (i.e., in `devtools::load_all()`).

## [v0.5.0] (2024-04-05)

### Breaking changes

* To support enum properly (the details follow), now savvy requires to put
  `#[savvy]` macro also on `struct`.

  ```rust
  #[savvy]   // NEW!
  struct Person {
      pub name: String,
  }
  
  #[savvy]
  impl Person {
  ```

  This might be a bit inconvenient on the one hand, but, on the other hand,
  several good things are introduced by this change! See the New Features
  section.

### New features

* Now `#[savvy]` macro supports enum to express the possible options for a
  parameter. This is useful when you want to let users specify some option
  without fear of typo. See [the guide](https://yutannihilation.github.io/savvy/guide/enum.html) for more details.

  Example:

  ```rust
  /// @export
  #[savvy]
  enum LineType {
      Solid,
      Dashed,
      Dotted,
  }

  /// @export
  #[savvy]
  fn plot_line(x: IntegerSexp, y: IntegerSexp, line_type: &LineType) -> savvy::Result<()> {
      match line_type {
          LineType::Solid => {
              ...
          },
          LineType::Dashed => {
              ...
          },
          LineType::Dotted => {
              ...
          },
      }
  }
  ```
  ```r
  plot_line(x, y, LineType$Solid)
  ```

* Savvy now allows `impl` definition over multiple files. It had been a headache
  that it wouldn't compile when you specified `#[savvy]` on `impl` of a same
  struct multiple times. But now, you can split the `impl` not only within a
  same file but also over multiple files.

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
  in an error. See [the guide](https://yutannihilation.github.io/savvy/guide/struct.html) for more details.
  
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
  guide](https://yutannihilation.github.io/savvy/guide/struct.html)
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
[Unreleased]: https://github.com/yutannihilation/savvy/compare/v0.6.8...HEAD
[v0.6.8]: https://github.com/yutannihilation/savvy/compare/v0.6.7...v0.6.8
[v0.6.7]: https://github.com/yutannihilation/savvy/compare/v0.6.6...v0.6.7
[v0.6.6]: https://github.com/yutannihilation/savvy/compare/v0.6.5...v0.6.6
[v0.6.5]: https://github.com/yutannihilation/savvy/compare/v0.6.4...v0.6.5
[v0.6.4]: https://github.com/yutannihilation/savvy/compare/v0.6.3...v0.6.4
[v0.6.3]: https://github.com/yutannihilation/savvy/compare/v0.6.2...v0.6.3
[v0.6.2]: https://github.com/yutannihilation/savvy/compare/v0.6.1...v0.6.2
[v0.6.1]: https://github.com/yutannihilation/savvy/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/yutannihilation/savvy/compare/v0.5.3...v0.6.0
[v0.5.3]: https://github.com/yutannihilation/savvy/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/yutannihilation/savvy/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/yutannihilation/savvy/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/yutannihilation/savvy/compare/v0.4.2...v0.5.0
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
