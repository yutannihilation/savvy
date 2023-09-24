## Getting Started

See [Getting Started section of the GitHub repo's README][getting-started].

[getting-started]: https://github.com/yutannihilation/savvy/blob/master/README.md#getting-started

## Treating External SEXP and owned SEXP differently

Savvy is opinionated in many points. One thing I think should be explained
before diving into the details is that savvy uses separate types for SEXP passed
from outside and that created within Rust function. The former, external SEXP,
is read-only, and the latter, owned SEXP, is writable. Here's the list:

| R type               | Read-only version | Writable version     |
|:---------------------|:------------------|:---------------------|
| `INTSXP` (integer)   | [`IntegerSxp`]    | [`OwnedIntegerSxp`]  |
| `REALSXP` (numeric)  | [`RealSxp`]       | [`OwnedRealSxp`]     |
| `LGLSXP` (logical)   | [`LogicalSxp`]    | [`OwnedLogicalSxp`]  |
| `STRSXP` (character) | [`StringSxp`]     | [`OwnedStringSxp`]   |
| `VECSXP` (list)      | [`ListSxp`]       | [`OwnedListSxp`]     |

You might wonder why this is needed when we can just use `mut` to distinguish
the difference of mutability. I mainly had two motivations for this:

1. **avoid unnecessary protection**: an external SEXP are already protected by
   the caller, while an owned SEXP needs to be protected by ourselves.
2. **avoid unnecessary ALTREP checks**: an external SEXP can be ALTREP, so it's
   better to handle them in ALTREP-aware way, while an owned SEXP is not.

This would be a bit lengthy, so let's skip here. You can read the details on [my
blog post][blog1]. But, one correction is that I found the second reason might
not be very important because a benchmark showed it's more efficient to be
non-ALTREP-aware in most of the cases. Actually, the current implementation of
savvy is non-ALTREP-aware for int, real, and logical (See [#18][issue18]).

[blog1]: https://yutani.rbind.io/post/intro-to-savvy-part1/
[issue18]: https://github.com/yutannihilation/savvy/issues/18

## Basic rule

This is a simple Rust function to add the specified suffix to the input
character vector. `#[savvy]` macro turns this into an R function.

```no_run
#[savvy]
fn add_suffix(x: StringSxp, y: &str) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na());
            continue;
        }

        out.set_elt(i, &format!("{e}_{y}"));
    }

    Ok(out.into())
}
```

Let's look at the details one by one.

### `#[savvy]` macro

(`#[savvy]` macro can also be used for `impl` for a `struct`, but let's focus on
function's case for now.)

If you mark a funtion with `#[savvy]` macro, the corresponding implementations are generated:

1. Rust functions
    a. a wrapper function to handle Rust and R errors gracefully
    b. a function with the original body and some conversion from raw `SEXP`s to savvy types.
2. C function signature for the Rust function
3. C implementation for bridging between R and Rust
4. R implementation

For example, the above implementation generates the following codes.

Rust functions:

```no_run
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn add_suffix(x: savvy::SEXP, y: savvy::SEXP) -> savvy::SEXP {
    savvy::handle_result(savvy_add_suffix_inner(x, y))
}

unsafe fn savvy_add_suffix_inner(x: savvy::SEXP, y: savvy::SEXP) -> savvy::Result<savvy::SEXP> {
    let x = <savvy::RealSxp>::try_from(savvy::Sxp(x))?;
    let y = <&str>::try_from(savvy::Sxp(y))?;
    
    // ...original body...

}
```

C function signature:

```text
SEXP add_suffix(SEXP x, SEXP y);
```

C implementation (let's skip the details about `handle_result` for now):

```text
SEXP add_suffix__impl(SEXP x, SEXP y) {
    SEXP res = add_suffix(x, y);
    return handle_result(res);
}
```

R implementation:

```text
add_suffix <- function(x, y) {
  .Call(add_suffix__impl, x, y)
}
```

### Input and Output of savvy-able functions

The example function above has this signature.

```no_run
fn add_suffix(x: StringSxp, y: &str) -> savvy::Result<savvy::SEXP> {}
```

As you can guess, with `#[savvy]` macro cannot be applied to arbitrary
functions. The function must satisfy the following conditions

* The function's inputs are either non-owned savvy types (e.g., [`IntegerSxp`]
  and [`RealSxp`]) or corresponding Rust types for scalar (e.g., `i32` and `f64`).
* The function returns `savvy::Result<savvy::SEXP>` or nothing (in the latter
  case, an invisible `NULL` will be returned instead).


### How to read the values from input R objects

Basically, there are two ways to access the values. [`IntegerSxp`] and
[`RealSxp`] have more convenient way, and [`ListSxp`]'s interface is a bit
different. But, let's talk about it later, not here.

#### 1. `iter()`

[`IntegerSxp`], [`RealSxp`], [`LogicalSxp`], and [`StringSxp`] provide `iter()`
method so that you can access to the value one by one. This can be efficient
when the data is too large to copy.

```no_run
for (i, e) in x.iter().enumerate() {
    // ...snip...
}
```

#### 2. `to_vec()`

The types above also provide `to_vec()`. As the name indicates, this copies
values to a Rust vector. Copying can be costly for big data, but a vector is
handy if you need to pass the data around among Rust functions.

```no_run
let mut v = x.to_vec();
some_function_takes_vec(v);
another_function_takes_slice(v.as_slice());
```

You can think of copying cost as "import tax" on crossing the FFI boundary. If
you think it's worth, you should pay, and if not, you should not.

### How to prepare output R object

As you saw above, an owned SEXP can be allocated by using
`Owned{type}Sxp::new()`. `new()` takes the length of the vector as the argument.
If you need the same length of vector as the input, you can pass the `len()` of
the input `SEXP`.

```no_run
let mut out = OwnedStringSxp::new(x.len());
```

Values can be written on it by `set_elt()` one by one.

```no_run
for (i, e) in x.iter().enumerate() {
    // ...snip...

    out.set_elt(i, &format!("{e}_{y}"));
}
```

Then, you can convert it to [`SEXP`] by `into()`

```no_run
Ok(out.into())
```

### Missing values

There's no concept of "missing value" on the corresponding types of `Rust`. So,
it looks a normal value to Rust's side. But, the good news is that R uses the
sentinel values to represent `NA`, so it's possible to check if a value is `NA`
to R in case the type is either `i32`, `f64` or `&str`.

* `i32`: [The minimum value of `int`][na_int] is used for representing `NA`.
* `f64`: [A special value][na_real] is used for representing `NA`.
* `&str`: [A `CHARSXP` of string `"NA"`][na_string] is used for representing
  `NA`; this cannot be distinguished by comparing the content of the string, but
  we can compare the pointer address of the underlying C `char` array.

[na_int]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/arithmetic.c#L143
[na_real]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/arithmetic.c#L90-L98
[na_string]: https://github.com/wch/r-source/blob/ed51d34ec195b89462a8531b9ef30b7b72e47204/src/main/names.c#L1219

You can check if the value is `NA` by `is_na()`, and refer to the sentinel value
of `NA` by `<T>::na()`. If you care about missing values, you always have to
have a `if` branch for missing values like below. Otherwise, you will get a
character `"NA_suffix"`, not `NA_character_`, on the R session.

```no_run
for (i, e) in x.iter().enumerate() {
    if e.is_na() {
        out.set_elt(i, <&str>::na());
        continue;
    }

    out.set_elt(i, &format!("{e}_{y}"));
}
```

The bad news is that `bool` is not the case. `bool` doesn't have `is_na()` or
`na()`. `NA` is treated as `TRUE`. So, you have to make sure the input doesn't
contain any missing values on R's side. So, for example, this function is not an
identity function.

```no_run
#[savvy]
fn identity_logical(x: LogicalSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e);
    }

    Ok(out.into())
}
```

```text
> identity_logical(c(TRUE, FALSE, NA))
[1]  TRUE FALSE  TRUE
```

#### Scalar input

If the type of the input is scalar, `NA` is always rejected. This is
inconsistent with the rule for vector input, but, this is my design decision in
the assumption that a scalar missing value is rarely found useful on Rust's
side. In this case, `bool` is also `NA`-aware.

```no_run
#[savvy]
fn identity_logical_single(x: bool) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedLogicalSxp::new(1);
    out.set_elt(0, x);
    Ok(out.into())
}
```

```text
> identity_logical_single(NA)
Error in identity_logical_single(NA) : 
  Must be length 1 of non-missing value
```

### No implicit conversion

savvy doesn't provide conversion between types. For example, you cannot supply a
numeric vector to a function with a `IntegerSxp` argument.

```no_run
#[savvy]
fn identity_int(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len());

    for (i, &v) in x.iter().enumerate() {
        out[i] = v;
    }

    Ok(out.into())
}
```

```text
> identity_int(c(1, 2))
Error in identity_int(c(1, 2)) : 
  Unexpected type: Cannot convert double to integer
```

While you probably feel this is inconvenient, this is also a design decision.
My concerns on supporting these conversion are

* Complexity. It would make savvy's spec and implemenatation complicated.
* Hidden allocation. Conversion requires a new allocation for storing the
  converted values, which might be unhappy in some cases.

So, you have to write some wrapper R function like below. This might feel a bit
tiring, but, in general, **please do not avoid R code**. Since you are creating an R
package, there's a lot you can do in R code instead of making things complicated
in Rust code. Especially, it's easier on R's side to show user-friendly error
messages.

```text
identity_int_wrapper <- function(x) {
    identity_int(vctrs::vec_cast(x, integer()))
}
```

## Integer and real

In cases of integer (`IntegerSxp`, `OwnedIntegerSxp`) and real (`RealSxp`,
`OwnedRealSxp`), the internal representation of the SEXPs match with the Rust
type we expect, i.e., `i32` and `f64`. By taking this advantage, these types has
more methods than other types:

* `as_slice()` and `as_mut_slice()`
* `Index` and `IndexMut`
* `From<&[T]>` trait

### `as_slice()` and `as_mut_slice()`

These types can expose its underlying C array as a Rust slice by `as_slice()`.
`as_mut_slice()` is available only for the owned versions. So, you don't need to
use `to_vec()` to create a new vector just to pass the data to the function that
requires slice. 

```no_run
#[savvy]
fn foo(x: IntegerSxp) {
    some_function_takes_slice(x.as_slice());
}
```

### `Index` and `IndexMut`

You can also access to the underlying data by `[`. `IndexMut` is available only
for the owned versions. This means you can write assignment operation like below
instead of `set_elt()`.

```no_run
#[savvy]
fn times_two(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len());

    for (i, &v) in x.iter().enumerate() {
        out[i] = v * 2;
    }

    Ok(out.into())
}
```

### `From<&[T]>` trait

As the owned versions implement `From<&[T]>`, you can use a Rust vector to store
results and convert it to an R object in the end. So, you can rewrite the
function above like below:

```no_run
#[savvy]
fn times_two(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out: Vec<i32> = Vec::with_capacity(x.len());

    for &v in x.iter() {
        out.push(v * 2);
    }

    let out_sxp: OwnedIntegerSxp = out.as_slice().into();
    Ok(out_sxp.into())
}
```

Under the hood, this uses [`copy_from_slice`][copy_from_slice], which does a
`memcpy`. So, this is more efficient than copying values one by one.

[copy_from_slice]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice


## Logical

TBD


## String


## List

TBD

## Struct

TODO: write about the need of protection if the field is SEXP.

TBD

## Use the raw R's C API (libR-sys)

### `unwind_protect()`

TBD

## How to use multiple Rust files

TBD