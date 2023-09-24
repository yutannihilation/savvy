## External SEXP and owned SEXP

First thing I want to discuss is that savvy uses separate types for SEXP passed
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

This is a simple function to add the specified suffix to the input character
vector.

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

Let's look at the lines one by one.

### `#[savvy]` macro

If you mark a funtion with `#[savvy]` macro, the corresponding implementations are generated:

1. Rust functions
    a. a wrapper function to handle Rust and R errors gracefully
    b. a function with the original body and some conversion from raw [`SEXP`]s to savvy types.
2. C signature for the header file
3. C implementation
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

C signature for the header file:

```text
SEXP add_suffix(SEXP x, SEXP y);
```

C implementation:

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

```no_run
fn add_suffix(x: StringSxp, y: &str) -> savvy::Result<savvy::SEXP> {...}
```

The function must satisfy the following conditions

* The function's inputs are either non-owned savvy types (e.g., [`IntegerSxp`]
  and [`RealSxp`]) or corresponding Rust types for scalar (e.g., `i32` and `f64`).
* The function returns `savvy::Result<savvy::SEXP>` or nothing (in the latter
  case, an invisible `NULL` will be returned instead).

(`#[savvy]` macro can also be used for `impl` for a `struct`. Let's revisit later.)

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

The types above also provides `to_vec()`. As the name indicates, this copies
values to a Rust vector. Copying can be an overhead, but this is handy if you
need to pass the data around among Rust functions.

```no_run
let mut v = x.to_vec();
some_function_takes_vec1(v);
some_function_takes_vec2(v);
```

### How to prepare output R object

As you saw above, an owned SEXP can be allocated by using `Owned{type}Sxp::new()`.

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

TBD


## Integer and real

The integer types (`IntegerSxp`, `OwnedIntegerSxp`) and the real types
(`RealSxp`, `OwnedRealSxp`) are easy in that the internal types of the SEXPs
match with the type we expect. By taking this advantage, these types has more
methods than other types:

* `as_slice()`
* `as_mut_slice()` for the owned versions

### `Index` and `IndexMut` trait for the owneed versions

So, for example, you can write



### `From<&[T]>` trait for the owned versions

TBD

## Logical

TBD


## List

TBD

## struct

TODO: write about the need of protection if the field is SEXP.

TBD

## How to use multiple Rust files

TBD