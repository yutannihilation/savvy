# Integer, Real, String, Bool, And Complex

## Integer and real

In cases of integer (`IntegerSexp`, `OwnedIntegerSexp`) and real (`RealSexp`,
`OwnedRealSexp`), the internal representation of the SEXPs match with the Rust
type we expect, i.e., `i32` and `f64`. By taking this advantage, these types has
more methods than other types:

* `as_slice()` and `as_mut_slice()`
* `Index` and `IndexMut`
* efficient `TryFrom<&[T]>`

### `as_slice()` and `as_mut_slice()`

These types can expose its underlying C array as a Rust slice by `as_slice()`.
`as_mut_slice()` is available only for the owned versions. So, you don't need to
use `to_vec()` to create a new vector just to pass the data to the function that
requires slice. 

```rust
/// @export
#[savvy]
fn foo(x: IntegerSexp) -> savvy::Result<()> {
    some_function_takes_slice(x.as_slice());
    Ok(())
}
```

### `Index` and `IndexMut`

You can also access to the underlying data by `[`. These methods are available
only for the owned versions. This means you can write assignment operation like
below instead of `set_elt()`.

```rust
/// @export
#[savvy]
fn times_two(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, &v) in x.iter().enumerate() {
        out[i] = v * 2;
    }

    out.into()
}
```

### Efficient `TryFrom<&[T]>`

`TryFrom<&[T]>` is not special to real and integer, but the implementation is
different from that of logical and string; since the internal representations
are the same, savvy uses [`copy_from_slice()`][copy_from_slice], which does a
`memcpy`, to copy the data efficently (in logical and string case, the values
are copied one by one).

[copy_from_slice]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice


## Logical

While logical is 3-state (`TRUE`, `FALSE` and `NA`) on R's side, `bool` can
represent only 2 states (`true` and `false`). This mismatch is a headache. There
are many possible ways to handle this (e.g., use `Option<bool>`), but savvy
chose to convert `NA` to `true` silently, assuming `NA` is not useful on Rust's
side anyway. So, you have to make sure the input logical vector doesn't contain
`NA` on R's side. For example,

```r
wrapper_of_some_savvy_fun <- function(x) {
  out <- rep(NA, length(x))
  idx <- is.na(x)

  # apply function only non-NA elements
  out[x] <- some_savvy_fun(x[idx])

  out
}
```

If you really want to handle the 3 states, use an expert-only method
`as_slice_raw()`. This returns `&[i32]` instead of `&[bool]`. Why `i32`? It's
the internal representation of a logical vector, which is the same as an integer
vector. By treating the data as `i32`, you can use `is_na()`.

```rust
use savvy::NotAvailableValue;   // for is_na()

/// @export
#[savvy]
fn flip_logical_expert_only(x: LogicalSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedLogicalSexp::new(x.len())?;

    for (i, e) in x.as_slice_raw().iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
        } else {
            out.set_elt(i, *e != 1)?; // 1 means TRUE
        }
    }

    out.into()
}
```

```r
flip_logical_expert_only(c(TRUE, FALSE, NA))
#> [1]  TRUE FALSE    NA
```

## String

`STRSXP` is a vector of `CHARSXP`, not something like `*char`. So, it's not
possible to expose the internal representation as `&str`. So, it requires
several R's C API calls. To get a `&str`

1. `STRING_ELT()` to subset a `CHARSXP`
2. `R_CHAR()` to extract the string from `CHARSXP`

Similarly, to set a `&str`

1. `Rf_mkCharLenCE()` to convert `&str` to a `CHARSEXP`
2. `SET_STRING_ELT()` to put the `CHARSXP` to the `STRSXP`

This is a bit costly. So, if the strings need to be referenced and updated
frequently, probably you should avoid using `OwnedStringSexp` as a substitute of
`Vec<String>`.

### Encoding and `'static` lifetime

While Rust's string is UTF-8, R's string is not guaranteed to be UTF-8. R
provides `Rf_translateCharUTF8()` to convert the string to UTF-8. However, savvy
chose not to use it. There are two reasons:

1. As of version 4.2.0, R uses UTF-8 as the native encoding even on Windows
   systems. While old Windows systems are not the case, I bravely assumes it's
   rare and time will solve.
2. The result of `R_CHAR()` is the string stored in `R_StringHash`, [the global
   `CHARSXP` cache][charsxp-cache]. In my understanding, this will never be
   removed during the session. So, this allows savvy to mark the result `&str`
   with `'static` lifetime. However, the result of `Rf_translateCharUTF8()` is
   on an `R_alloc()`-ed memory ([code][Rf_translateCharUTF8]), which can be
   claimed by GC.
   
In short, in order to stick with `'static` lifetime for the sake of simplicity,
I decided to neglect relatively-rare case. Note that, invalid UTF-8 charactars
are rejected (= currently, silently replaced with `""`) by `CStr`, so it's not
very unsafe.

[charsxp-cache]: https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#The-CHARSXP-cache
[Rf_translateCharUTF8]: https://github.com/wch/r-source/blob/c3423d28830acbbbf7b38daa58f436fb06d91381/src/main/sysutils.c#L1284-L1296


## Complex

Complex is optionally supported under feature flag `complex`. If it's enabled,
you can use `ComplexSexp` and `OwnedComplexSexp` to use a complex vector for
input or output, and you can extract the slice of `num_complex::Complex64` from
it.

```rust
/// @export
#[savvy]
fn abs_complex(x: savvy::ComplexSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = savvy::OwnedRealSexp::new(x.len())?;

    for (i, c) in x.iter().enumerate() {
        if !c.is_na() {
            out[i] = (c.re * c.re + c.im * c.im).sqrt();
        } else {
            out.set_na(i)?;
        }
    }

    out.into()
}
```