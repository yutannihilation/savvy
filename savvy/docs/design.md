## Getting Started

See [Getting Started section of README][getting-started].

[getting-started]: https://crates.io/crates/savvy#getting-started

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
    let mut out = OwnedStringSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na())?;
            continue;
        }

        out.set_elt(i, &format!("{e}_{y}"))?;
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
    match savvy_add_suffix_inner(x, y) {
        Ok(result) => result,
        Err(e) => savvy::handle_error(e),
    }
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
fn add_suffix(x: StringSxp, y: &str) -> savvy::Result<savvy::SEXP>
```

As you can guess, `#[savvy]` macro cannot be applied to arbitrary functions. The
function must satisfy the following conditions:

* The function's inputs are either non-owned savvy types (e.g., [`IntegerSxp`]
  and [`RealSxp`]) or corresponding Rust types for scalar (e.g., `i32` and `f64`).
* The function returns `savvy::Result<savvy::SEXP>` or `savvy::Result<()>` (in
  the latter case, an invisible `NULL` will be returned instead).

For more flexibility, I plan to support raw `SEXP` as input, but it's not yet.

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

#### 1. `new()`

As you saw above, an owned SEXP can be allocated by using
`Owned{type}Sxp::new()`. `new()` takes the length of the vector as the argument.
If you need the same length of vector as the input, you can pass the `len()` of
the input `SEXP`.

`new()` returns [`Result`] because the memory allocation can fail in case
when the vector is too large. If you are sure it won't happen, you can simply
`unwrap()` it. If you use `new()` directly in the function marked with
`#[savvy]`, it's as easy as just adding `?` because the return type is always
`savvy::Result<SEXP>`.

```no_run
let mut out = OwnedStringSxp::new(x.len())?;
```

Values can be written on it by `set_elt()` one by one.

```no_run
for (i, e) in x.iter().enumerate() {
    // ...snip...

    out.set_elt(i, &format!("{e}_{y}"))?;
}
```

Then, you can convert it to [`SEXP`] by `into()`

```no_run
Ok(out.into())
```

#### 2. `try_into()`

Another way is to use a Rust vector to store the results and convert it to an R
object at the end the function. This is fallible because this too uses the
fallible `new()`.

Note that, while this is convenient, this might not be good in terms of
efficiency in that this requires double size of memory.

```no_run
#[savvy]
fn times_two(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out: Vec<i32> = Vec::with_capacity(x.len());

    for &v in x.iter() {
        out.push(v * 2);
    }

    let out_sxp: OwnedIntegerSxp = out.as_slice().try_into()?;
    Ok(out_sxp.into())
}
```

Note that, the efficiency of copying differs depending on the types. For the
details, see `TryFrom<&[T]>` section later.

For convenience, savvy also provides `TryFrom` for scalar types. This might be
useful in some cases.

```no_run
#[savvy]
fn sum_real(x: RealSxp) -> savvy::Result<savvy::SEXP> {
    let sum: OwnedRealSxp = x.as_slice().iter().sum::<f64>().try_into()?;
    Ok(sum.into())
}
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
        out.set_elt(i, <&str>::na())?;
        continue;
    }

    out.set_elt(i, &format!("{e}_{y}"))?;
}
```

The bad news is that `bool` is not the case. `bool` doesn't have `is_na()` or
`na()`. `NA` is treated as `TRUE` without any errors. So, you have to make sure
the input doesn't contain any missing values on R's side. For example, this
function is not an identity function.

```no_run
#[savvy]
fn identity_logical(x: LogicalSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedLogicalSxp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        out.set_elt(i, e)?;
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
    let mut out = OwnedLogicalSxp::new(1)?;
    out.set_elt(0, x)?;
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
    let mut out = OwnedIntegerSxp::new(x.len())?;

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
tiring, but, in general, **please do not avoid writing R code**. Since you are
creating an R package, there's a lot you can do in R code instead of making
things complicated in Rust code. Especially, it's easier on R's side to show
user-friendly error messages.

```text
identity_int_wrapper <- function(x) {
  x <- vctrs::vec_cast(x, integer())
  identity_int(x)
}
```

## Integer and real

In cases of integer (`IntegerSxp`, `OwnedIntegerSxp`) and real (`RealSxp`,
`OwnedRealSxp`), the internal representation of the SEXPs match with the Rust
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

```no_run
#[savvy]
fn foo(x: IntegerSxp) -> savvy::Result<()> {
    some_function_takes_slice(x.as_slice());
    Ok(())
}
```

### `Index` and `IndexMut`

You can also access to the underlying data by `[`. These methods are available
only for the owned versions. This means you can write assignment operation like
below instead of `set_elt()`.

```no_run
#[savvy]
fn times_two(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len())?;

    for (i, &v) in x.iter().enumerate() {
        out[i] = v * 2;
    }

    Ok(out.into())
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

```text
wrapper_of_some_savvy_fun <- function(x) {
  out <- rep(NA, length(x))
  idx <- is.na(x)

  # apply function only non-NA elements
  out[x] <- some_savvy_fun(x[idx])

  out
}
```

If you really want to handle the 3 states, use `IntegerSxp` as the argument type
and convert the logical into an integer before calling the savvy function. To
represent 3-state, the internal representation of `LGLSXP` is int, which is the
same as `INTSXP`. So, the conversion should be cheap.

```no_run
#[savvy]
fn some_savvy_fun(logical: IntegerSxp) -> savvy::Result<()> {
    for l in logical.iter() {
        if l.is_na() {
            r_print("NA\n");
        } else if *l == 1 {
            r_print("TRUE\n");
        } else {
            r_print("FALSE\n");
        }
    }

    Ok(())
}
```

```text
wrapper_of_some_savvy_fun <- function(x) {
  x <- vctrs::vec_cast(x, integer())
  some_savvy_fun(x)
}
```

## String

`STRSXP` is a vector of `CHARSXP`, not something like `*char`. So, it's not
possible to expose the internal representation as `&str`. So, it requires
several R's C API calls. To get a `&str`

1. `STRING_ELT()` to subset a `CHARSXP`
2. `Rf_translateCharUTF8()` to extract a UTF-8 string from `CHARSXP`

Similarly, to set a `&str`

1. `Rf_mkCharLenCE()` to convert `&str` to a `CHARSEXP`
2. `SET_STRING_ELT()` to put the `CHARSXP` to the `STRSXP`

This is a bit costly. So, if the strings need to be referenced and updated
frequently, probably you should avoid using `OwnedStringSxp` as a substitute of
`Vec<String>`.

## List

List is a different beast. It's pretty complex. You might think of it as a
`HashMap`, but it's different in that:

* List elements can be either named or unnamed individually (e.g., `list(a = 1,
  2, c = 3)`).
* List names can be duplicated (e.g., `list(a = 1, a = 2)`).

To make things simple, savvy treats a list as a pair of the same length of

* a character vector containing names, using `""` (empty string) to represent
  missingness (actually, this is the convention of R itself)
* a collection of arbitrary `SEXP` elements

Since list is a very convenient data structure in R, you can come up with a lot
of convenient interfaces for list. However, savvy intentionally provides only
very limited interfaces. In my opinion, Rust should touch list data as little as
possible because it's too complex.

### How to read values from a list

#### `names_iter()`

`names_iter()` returns an iterator of `&str`.

```no_run
#[savvy]
fn print_list_names(x: ListSxp) -> savvy::Result<()> {
    for k in x.names_iter() {
        if k.is_empty() {
            r_print("(no name)")?;
        } else {
            r_print(k)?;
        }
        r_print("\n")?;
    }

    Ok(())
}
```

```text
> print_list_names(list(a = 1, 2, c = 3))
a
(no name)
c
```

#### `values_iter()`

`values_iter()` returns an iterator of [`ListElement`] enum. You can use `match`
to extract the inner data.

```no_run
#[savvy]
fn print_list_values_if_int(x: ListSxp) -> savvy::Result<()>  {
    for v in x.values_iter() {
        match v {
            ListElement::Integer(i) => r_print(&format!("int {}\n", i.as_slice()[0]))?,
            _ => r_print("not int\n")?
        }
    }

    Ok(())
}
```

```text
> print_list_values_if_int(list(a = 1, b = 1L, c = "1"))
not int
int 1
not int
```

#### `iter()`

If you want pairs of name and value, you can use `iter()`. This is basically a
`std::iter::Zip` of the two iterators explained above.

```no_run
#[savvy]
fn print_list(x: ListSxp)  -> savvy::Result<()> {
    for (k, v) in x.iter() {
        // ...snip...
    }

    Ok(())
}
```

### How to put values to a list

#### `new()`

[`OwnedListSxp`]'s `new()` is different than other types; the second argument
(`named`) indicates whether the list is named or unnamed. If `false`, the list
doesn't have name and all operations on name like `set_name()` are simply
ignored.

#### `set_name()`

`set_name()` simply sets a name at the specified position.

```no_run
#[savvy]
fn list_with_no_values() -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedListSxp::new(2, true)?;

    out.set_name(0, "foo");
    out.set_name(1, "bar");

    Ok(out.into())
}
```
```text
> list_with_no_values()
$foo
NULL

$bar
NULL

```

#### `set_value()`

`set_value()` sets a value at the specified position. "Value" is an arbitrary
type that implmenents `Into<ListElement>` trait. Since all `{type}Sxp` types
implements it, you can simply pass it like below.

```no_run
#[savvy]
fn list_with_no_names() -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedListSxp::new(2, false)?;

    let mut e1 = OwnedIntegerSxp::new(1)?;
    e1[0] = 100;
    
    let mut e2 = OwnedStringSxp::new(1)?;
    e2.set_elt(0, "cool")?;

    out.set_value(0, e1);
    out.set_value(1, e2);

    Ok(out.into())
}
```
```text
> list_with_no_names()
[[1]]
[1] 100

[[2]]
[1] "cool"

```

#### `set_name_and_value()`

`set_name_and_value()` is simply `set_name()` + `set_value()`. Probably this is
what you need in most of the cases.

## Struct

You can use `#[savvy]` macro on `impl` for a `struct`. For example, this code

```no_run
struct Person {
    pub name: String,
}

#[savvy]
impl Person {
    fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    fn set_name(&mut self, name: &str) -> savvy::Result<()> {
        self.name = name.to_string();
        Ok(())
    }

    fn name(&self) -> savvy::Result<savvy::SEXP> {
        let mut out = OwnedStringSxp::new(1)?;
        out.set_elt(0, &self.name)?;
        Ok(out.into())
    }
}
```

allows you to use `Person` like below on R sessions. One special convention is
that, if the name of the method is `new`, it's used as the constructor function
(in this case, `Person()`).

```text
> x <- Person()
> x$set_name("たかし")
> x$name()
[1] "たかし"
```

### External pointer?

Under the hood, the `Person` struct is stored in `EXTPTRSXP`. But, there's no
corresponding type for `EXTPTRSXP` in savvy. Why? This is because it's stored in
closure environemnts on creation and never exposed to the user. As it's
guaranteed on R's side that `self` is always a `EXTPTRSXP` of `Person`, Rust
code just restore a `Person` instance from the `EXTPTRSXP` without any checks.
So, you can forget the details about external pointer.

```text
Person <- function() {
  e <- new.env(parent = emptyenv())
  self <- .Call(Person_new__impl)

  e$set_name <- Person_set_name(self)
  e$name <- Person_name(self)

  class(e) <- "Person"
  e
}

Person_set_name <- function(self) {
  function(name) {
    invisible(.Call(Person_set_name__impl, self, name))
  }
}

Person_name <- function(self) {
  function() {
    .Call(Person_name__impl, self)
  }
}
```

### Traps about protection

This is a bit advanced topic. It's okay to have a struct to contain arbitrary
things, however, if you want to pass an `SEXP` from an R session, you need to
take care of the protection on it.

The `SEXP` passed from outside doesn't need an additional protection at the time
of the function call because it belongs to some enviroment on R session, which
means it's not GC-ed accidentally. However, after the function call, it's
possible the `SEXP` loses its link to any other R objects. To prevent the
tragedy (i.e., R session crash), you should create a owned version and copy the
values into it because savvy takes care of the protection on it. So, in short,
you should never define such a struct like this:

```no_run
struct Foo {
    a: IntegerSxp
}
```

Instead, you should write

```no_run
struct Foo {
    a: OwnedIntegerSxp
}
```

## Advanced topics

### Error handling

To propagate your errors to the R session, you can use `savvy::Error::new()` to
create an error with a custom error message.

```no_run
#[savvy]
fn raise_error() -> savvy::Result<savvy::SEXP> {
    Err(savvy::Error::new("This is my custom error"))
}
```

```text
> raise_error()
Error: This is my custom error
```

### Testing


### Use the raw R's C API (libR-sys)

#### `unwind_protect()`

TBD

### How to use multiple Rust files

TBD

