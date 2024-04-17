# Key Ideas

## Treating external SEXP and owned SEXP differently

Savvy is opinionated in many points. Among these, one thing I think should be
explained first is that savvy uses separate types for SEXP passed from outside
and that created within Rust function. The former, external SEXP, is read-only,
and the latter, owned SEXP, is writable. Here's the list:

| R type                          | Read-only version       | Writable version     |
|:--------------------------------|:------------------------|:---------------------|
| `INTSXP` (integer)              | `IntegerSexp`           | `OwnedIntegerSexp`   |
| `REALSXP` (numeric)             | `RealSexp`              | `OwnedRealSexp`      |
| `LGLSXP` (logical)              | `LogicalSexp`           | `OwnedLogicalSexp`   |
| `STRSXP` (character)            | `StringSexp`            | `OwnedStringSexp`    |
| `VECSXP` (list)                 | `ListSexp`              | `OwnedListSexp`      |
| `EXTPTRSXP` (external pointer)  | `ExternalPointerSexp`   | n/a                  |
| `CPLXSXP` (complex)[^1]         | `ComplexSexp`           | `OwnedComplexSexp`   |

[^1]: Complex is optionally supported under feature flag `complex`

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

## No implicit conversions

Savvy doesn't provide conversion between types. For example, you cannot supply a
numeric vector to a function with a `IntegerSexp` argument.

```rust
#[savvy]
fn identity_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedIntegerSexp::new(x.len())?;

    for (i, &v) in x.iter().enumerate() {
        out[i] = v;
    }

    out.into()
}
```

``` r
identity_int(c(1, 2))
#> Error in identity_int(c(1, 2)) : 
#>   Unexpected type: Cannot convert double to integer
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

``` r
identity_int_wrapper <- function(x) {
  x <- vctrs::vec_cast(x, integer())
  identity_int(x)
}
```

Alternatively, you can use a general type `Sexp` as input and switch the
function to apply depending on whether it's integer or real.

```rust
#[savvy]
fn identity_num(x: Sexp) -> savvy::Result<savvy::Sexp> {
    match x.into_typed() {
        TypedSexp::Integer(i) => identity_int(i),
        TypedSexp::Real(r) => identity_real(r),
        _ => Err("Expected integer or numeric".into()),
    }
}
```