# Changelog

## [Unreleased]

### Breaking changes

* `r_print!` and `r_eprint!` are now macro that wraps `format!`, so you can use
  them just like Rust's `print!` macro. There are also `r_println!` and
  `r_eprintln!` available.

### New features

* Support scalar `usize` input.
* Add some methods to access the attributes of an input SEXP: `get_names()`,
  `get_class()`, and `get_dim()`.

### Fixed bugs

* Newly-created R vectors (`Owned*Sexp`) are now properly initialized. If you
  really want to skip the initialization for some great reason, you can use
  `new_without_init()` instead of `new()`.
