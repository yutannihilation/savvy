# Struct

You can use `#[savvy]` macro on `impl` for a `struct`. For example, this code

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
}
```

allows you to use `Person` like below on R sessions. One special convention is
that, if the name of the method is `new`, it's used as the constructor function
(in this case, `Person()`).

```r
x <- Person()
x$set_name("たかし")
x$name()
#> [1] "たかし"
```

## External pointer?

Under the hood, the `Person` struct is stored in `EXTPTRSXP`. But, there's no
corresponding type for `EXTPTRSXP` in savvy. Why? This is because it's stored in
closure environemnts on creation and never exposed to the user. As it's
guaranteed on R's side that `self` is always a `EXTPTRSXP` of `Person`, Rust
code just restore a `Person` instance from the `EXTPTRSXP` without any checks.
So, you can forget the details about external pointer.

```r
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

## Traps about protection

This is a bit advanced topic. It's okay to have a struct to contain arbitrary
things, however, if you want to pass an `SEXP` from an R session, **you need to
take care of the protection on it**.

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
