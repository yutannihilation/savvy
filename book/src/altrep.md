# ALTREP

You can implement an ALTREP class using savvy. 

## Disclaimer

* This feature is very experimental, so it's possible that the interface will be
  significantly changed or even removed in future.

* The current API might be a bit oversimplified. For example, you cannot stop
  the vector is materialized (i.e., allocated as a normal `SEXP` and put into
  the `data2` slot of the ALTREP object).

## Using ALTREP

Savvy currently provides only the following traits for ALTREP. The other ALTREPs
like `ALTCOMPLEX` are not yet supported.

* [`AltInteger`](https://yutannihilation.github.io/savvy/reference/savvy/altrep/trait.AltInteger.html)
* [`AltReal`](https://yutannihilation.github.io/savvy/reference/savvy/altrep/trait.AltReal.html)
* [`AltLogical`](https://yutannihilation.github.io/savvy/reference/savvy/altrep/trait.AltLogical.html)
* [`AltString`](https://yutannihilation.github.io/savvy/reference/savvy/altrep/trait.AltString.html)
* [`AltList`](https://yutannihilation.github.io/savvy/reference/savvy/altrep/trait.AltList.html)

For example, consider the following struct that simply wraps a `Vec<i32>`.

```rust
struct MyAltInt(Vec<i32>);

impl MyAltInt {
    fn new(x: Vec<i32>) -> Self {
        Self(x)
    }
}
```

First, you need to implement `IntoExtPtrSexp` trait for the struct, which is
required by `Alt*` traits. This trait is what works under the hood of `#[savvy]`
when it's placed on a struct. You can just rely on the default implementation.

```rust
impl savvy::IntoExtPtrSexp for MyAltInt {}
```

Second, you need to implement one of the `Alt*` traits. More specifically, the
trait has 4 members you need to implement:

* `CLASS_NAME` is the name of the class. This is used for distinguishing the class, so
  please use a unique string.
* `PACKAGE_NAME` is the name of your package. This probably doesn't matter much.
* `length()` returns the length of the object.
* `elt(i)` returns the `i`-th element of the object. An important note is that,
  usually R handles the out-of-bound check and returns `NA` if it exceeds the
  length. So, you don't need to check the length here.

In this case, the actual data is `i32`, so let's implement `AltInteger`.

``` rust
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

Optionally, you can implement these methods:

* `copy_date(dst, offset)`: This copies the range of values starting from
  `offset` into `dst`, a `&mut [T]`. The default implementation does just call
  `elt()` repeatedly, but there might be more efficient implementation (e.g.
  `copy_from_slice()`).
* `inspect()`: This is called when `.Internal(inspect(x))`. You might want to
  print some information useful for debugging.

Next step is a bit advanced. You need to create a definition of ALTREP class
from the above trait. This is done by the corresponding `register_alt*_class()`
function (for example, `register_altinteger_class` for an integer class). This
function generates an ALTREP class and registers it to an R session.

The registration needs to happen when an R session loads the DLL of your crate.
As explained in the section of [initialization routine](./initialization_routine.md),
you can define a `#[savvy_init]` function, which will be called in the 
initialization routine.

``` rust
#[savvy_init]
fn init_altrep_class(dll_info: *mut DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    Ok(())
}
```

Finally, you'll probably want to implement a user-visible function to create the
instance of the ALTREP class. You can convert the struct into an ALTREP by
`.into_altrep()` method, which is provided by the `Alt*` trait. For example, you
can create the following function that returns the length 3 of the ALTREP vector
to the R session.

``` rust
#[savvy]
fn altint() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![1, 2, 3]);
    v.into_altrep()
}
```

This function can be used like this:

``` r
x <- altint()

x
#> [1] 1 2 3
```

This looks like a normal integer vector, but this is definitely an ALTREP.

```r
.Internal(inspect(x))
#> @0x0000021684acac40 13 INTSXP g0c0 [REF(65535)] (MyAltInt)
```

## Going deeper...

Once the ALTREP object leaves your hand, it looks like a normal vector. But, if
you really wish, you can convert it back to the original object. `Alt*` trait
provides 3 methods for this conversion:

* `try_from_altrep_ref()` for `&T`
* `try_from_altrep_mut()` for `&mut T`
* `try_from_altrep()` for `T`

For example, you can print the underlying data using `Debug` trait.

``` rust
#[savvy]
fn print_altint(x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}
```

``` r
print_altint(x)
#> MyAltInt([1, 2, 3])
```

But, before getting excited, you need to be aware about the tricky nature of R.

First, your ALTREP object can be easily lost in the sea of copy-on-modify. For
example, if the object is get modified, it's no longer an ALTREP object.

``` r
x <- altint()

x[1L] <- 3L

print_altint(x)
#> Error: Not a known ALTREP
```

Second, this is much trickier. As there is `try_from_altrep_mut()`, you can
modify the underlying data. For example, you can mutiply each number by two.

``` rust
#[savvy]
fn tweak_altint(mut x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_mut(&mut x, false) {
        for i in x.0.iter_mut() {
            *i *= 2;
        }
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}
```

Let's confirm this function modifies the underlying data as expected.

``` r
x <- altint()
c(x) # This is for a side effect! Let's discuss later.
#> [1] 1 2 3

tweak_altint(x)

print_altint(x)
#> MyAltInt([2, 4, 6])
```

So far, so good. But, if you print `x`, you'll find the values are diverged
between Rust and R... Why can this happen?

``` r
x
#> [1] 1 2 3
```

This is because savvy's implementation caches the SEXP object converted from the
underlying data. It's can be costly if it creates a fresh SEXP object everytime
the R session requires, so the result is cached at the first time it's created
(in the above case, it's `c(x)`). As far as I know, most of the ALTREP
implementation adopt this caching strategy (more specifically, an ALTREP object
has two slots, `data1` and `data2`, and `data2` is usually used for the cache).

But, don't worry. `try_from_altrep_mut()` has a second argument,
`invalidate_cache`. You can set this to `true` to clear the cache.

```rust
#[savvy]
fn tweak_altint2(mut x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_mut(&mut x, true) {
      //                                                 ^^^^^
      //                                                   changed!
```

``` r
tweak_altint2(x)
print_altint(x)
#> MyAltInt([2, 4, 6])

x
#> [1] 2, 4, 6
```

This API is still experimental and I'm yet to find some nicer design. Feedback
is really appreciated!
