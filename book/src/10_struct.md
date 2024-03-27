# Struct

## Basic usage

You can use `#[savvy]` macro on `impl` for a `struct` to make it available from
R code.

```rust
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

    fn name(&self) -> savvy::Result<savvy::Sexp> {
        let mut out = OwnedStringSexp::new(1)?;
        out.set_elt(0, &self.name)?;
        out.into()
    }

    fn say_hello() -> savvy::Result<savvy::Sexp> {
        "Hello!".try_into()
    }
}
```

If we focus on the arguments, there are two types of functions here:

1. method: the first argument is `&self` or `&mut self` [^self] (`set_name()` and `name()`)
2. associated function: no `&self` or `&mut self` argument (`new()` and `say_hello()`)

[^self]: Note that consuming the struct (i.e., `self`) is not allowed

On an R session, associated functions are available as the element of the same
name of R object as the Rust type (in this case, `Person`).

```r
p <- Person$new()

Person$say_hello()
#> [1] "Hello"
```

Among these two associated functions, `new()` is a constructor which returns
`Self`. This creates an instance of the struct.

The instance has the methods. You can call them like below.

```r
# create an instance
p <- Person$new()

# call methods
p$set_name("たかし")
p$name()
#> [1] "たかし"
```

The instance has the same name of S3 class as the Rust type, so you can implement
S3 methods such as `print.<your struct>()` if necessary.

```r
class(p)
#> [1] "Person"
```

### Struct output

The above example uses `-> Self` as the return type of the associated function,
but it's not the only specification. You can wrap it with `savvy::Result<Self>`.

```rust
#[savvy]
impl Person {
    fn new_fallible() -> savvy::Result<Self> {
        let x = Self {
            name: "".to_string(),
        };
        Ok(x)
    }
}
```

More generally, you can specify an arbitrary struct marked with `#[savvy]` as
the return type. For example, you can create an instance of the struct outside
of `impl`,

```rust
#[savvy]
fn create_person() -> savvy::Result<Person> {
    let x = Self {
        name: "".to_string(),
    };
    Ok(x)
}
```

and you can generate another type of instance from an instance.

```rust
struct UpperPerson {
    pub name: String,
}

#[savvy]
impl Person {}

#[savvy]
impl Person {
    fn reborn_as_upper_person(&self) -> savvy::Result<UpperPerson> {
        let x = UpperPerson {
            name: self.name.to_uppercase(),
        };
        Ok(x)
    }
}
```

### Struct input

You can also use the struct as the argument of a `#[savvy]`-ed function. The
type must be specified either as `&T` or as `&mut T`, not as `T`.

```rust
#[savvy]
fn get_name_external(x: &Person) -> savvy::Result<savvy::Sexp> {
    x.name()
}
```
```r
get_name_external(x)
#> [1] "たかし"
```

## External pointer?

Under the hood, the `Person` struct is stored in `EXTPTRSXP`. But, you don't
need to care about how to deal with `EXTPTRSXP`. This is because it's stored in
a closure environemnt on creation and never exposed to the user. As it's
guaranteed on R's side that `self` is always a `EXTPTRSXP` of `Person`, Rust
code just restore a `Person` instance from the `EXTPTRSXP` without any checks.

```r
Person <- function() {
  e <- new.env(parent = emptyenv())
  self <- .Call(Person_new__impl)

  e$.ptr <- self
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

## Traps about protection

This is a bit advanced topic. It's okay to have a struct to contain arbitrary
things, however, if you want to pass an `SEXP` from an R session, **it's your
responsibility to take care of the protection on it**.

The `SEXP` passed from outside doesn't need an additional protection at the time
of the function call because it belongs to some enviroment on R session, which
means it's not GC-ed accidentally. However, after the function call, it's
possible the `SEXP` loses its link to any other R objects. To prevent the
tragedy (i.e., R session crash), you should create a owned version and copy the
values into it because savvy takes care of the protection on it. So, in short,
you should never define such a struct like this:

```rust
struct Foo {
    a: IntegerSexp
}
```

Instead, you should write

```rust
struct Foo {
    a: OwnedIntegerSexp
}
```
