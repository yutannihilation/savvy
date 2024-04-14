# `#[savvy]` macro

This is a simple Rust function to add the specified suffix to the input
character vector. `#[savvy]` macro turns this into an R function.

```rust
use savvy::NotAvailableValue;   // for is_na() and na()

/// Add Suffix
/// 
/// @export
#[savvy]
fn add_suffix(x: StringSexp, y: &str) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
            continue;
        }

        out.set_elt(i, &format!("{e}_{y}"))?;
    }

    out.into()
}
```

## Convention for a `#[savvy]` function

The example function above has this signature.

```rust
fn add_suffix(x: StringSexp, y: &str) -> savvy::Result<savvy::Sexp>
```

As you can guess, `#[savvy]` macro cannot be applied to arbitrary functions. The
function must satisfy the following conditions:

* The function's inputs can be
    * a non-owned savvy type (e.g., `IntegerSexp` and `RealSexp`)
    * a corresponding Rust type for scalar (e.g., `i32` and `f64`)
    * a user-defined struct marked with `#[savvy]` (`&T`, `&mut T`, or `T`)
    * a user-defined enum marked with `#[savvy]` (`&T`, or `T`)
* The function's return value must be either
    * `savvy::Result<()>` for the case of no actual return value
    * `savvy::Result<savvy::Sexp>` for the case of some return value of R object
    * `savvy::Result<T>` for the case of some return value of a user-defined
      struct or enum marked with `#[savvy]`

## How things work under the hood

If you mark a funtion with `#[savvy]` macro, the corresponding implementations are generated:

1. Rust functions
    1. a wrapper function to handle Rust and R errors gracefully
    2. a function with the original body and some conversion from raw `SEXP`s to savvy types.
2. C function signature for the Rust function
3. C implementation for bridging between R and Rust
4. R implementation

For example, the above implementation generates the following codes. (`#[savvy]`
macro can also be used on `struct` and `enum`, but let's focus on function's
case for now for simplicity.)

### Rust functions

(The actual code is a bit more complex to handle possible `panic!` properly.)

```rust
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn add_suffix(x: SEXP, y: SEXP) -> SEXP {
    match savvy_add_suffix_inner(x, y) {
        Ok(result) => result.0,
        Err(e) => savvy::handle_error(e),
    }
}

unsafe fn savvy_add_suffix_inner(x: SEXP, y: SEXP) -> savvy::Result<savvy::Sexp> {
    let x = <savvy::RealSexp>::try_from(savvy::Sexp(x))?;
    let y = <&str>::try_from(savvy::Sexp(y))?;
    
    // ...original body...

}
```

### C function signature

```c
SEXP add_suffix(SEXP x, SEXP y);
```

### C implementation

(let's skip the details about `handle_result` for now)

```c
SEXP add_suffix__impl(SEXP x, SEXP y) {
    SEXP res = add_suffix(x, y);
    return handle_result(res);
}
```

### R implementation

The Rust comments with three slashes (`///`) is converted into Roxygen comments
on R code.

```r
#' Add Suffix
#' 
#' @export
add_suffix <- function(x, y) {
  .Call(add_suffix__impl, x, y)
}
```

## Using `#[savvy]` on other files than `lib.rs`

You can use `#[savvy]` macro just the same as `lib.rs`. One thing you have to do
is that you need to export the objects by `*`. This is because `#[savvy]`
defines additional functions other than the original ones, and some of these
also need to be exported. Since you don't know the names of such auto-generated
functions, `*` is the solution.

```rust
mod foo;
pub use foo::*;
```
