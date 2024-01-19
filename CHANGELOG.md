# Changelog

<!-- next-header -->
## [Unreleased] (ReleaseDate)

### Breaking changes

* `ListSexp` now returns an `Sexp` instead of a `TypedSexp`. Use `.into_typed()`
  to convert an `Sexp` to a `TypedSexp`.

### New features

* Add `is_null()`.
* Add `as_read_only()` to `OwnedListSexp` as well.
* Add `downcast()` for casting an external pointer to a concrete type. Note that
  this is only needed for "external" external pointers.

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
[Unreleased]: https://github.com/yutannihilation/savvy/compare/v0.2.4...HEAD
[v0.2.4]: https://github.com/yutannihilation/savvy/compare/savvy-v0.2.2...v0.2.4
[v0.2.2]: https://github.com/yutannihilation/savvy/compare/savvy-v0.2.1...savvy-v0.2.2
