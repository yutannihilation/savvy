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

1. method: the first argument is `self` [^self] (`set_name()` and `name()`)
2. associated function: no `self` argument (`new()` and `say_hello()`)

[^self]: You should almost always use `&self` or `&mut self`, not `self`, except
    when you are an expert and your intention is really to comsume it. Let's
    discuss later.

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

# register print() S3 method for Person
print.Person <- function(x, ...) print(x$name())
registerS3method("print", "Person", print.Person)

p
#> たかし
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

You can also use the struct as the argument of a `#[savvy]`-ed function. Note
that, in most of the cases, you should specify `&T` or `&mut T`, not `T`.

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

### `&T` vs `T`

If you are familiar with Rust, you should know the difference. `T` moves the
ownership while `&T` is just borrowing. But, what does this matter savvy? What
happens in actual when you specify `T` in a `#[savvy]` function?

Say, you mistyped `&Person` above as `Person` like this:

```rust
#[savvy]
fn get_name_external2(x: Person) -> savvy::Result<savvy::Sexp> {
    x.name()
}
```

This function works the same as the previous one. The result of the first call
is the same. Yay!

```r
get_name_external2(p)
#> [1] "たかし"
```

Then, what's wrong? You'll find it when you call the function on the same object
second time; it doesn't work anymore.

```r 
get_name_external2(p)
#> Error: This external pointer is already consumed or deleted
```

This is because the `Person` object is already moved. The R variable `p` doesn't
hold the ownership anymore. So, you should almost always specify `&T` (or `&mut T`),
not `T`. 

The same is true for a method. Use `&self` and `&mut self` instead of `self`
unless you want such a method like this!

```rust
#[savvy]
impl Person {
    fn invalidate(self) -> savvy::Result<()> {
        r_println!("This instance is invalidated!");
        Ok(())
    }
}
```

### When is `T` useful?

You might wonder why savvy allows this specification at all. Are there any cases
when this is useful?

The answer is yes. The advantage of moving the ownership is that you can avoid
copying. For example, consider there's a type `HeavyData`, which contains huge
size of data, and `HeavyDataBundle` which bundles two `HeavyData`s.

```rust
#[derive(Clone)]
struct HeavyData(Vec<i32>);

struct HeavyDataBundle {
    data1: HeavyData,
    data2: HeavyData,
}

#[savvy]
impl HeavyData {
    // ...snip...
}
```

`HeavyDataBundle` requires the ownership of the `DataBundle`s. So, if the input
is `&`, you need to `clone()` the data, which can be costly.

```rust
#[savvy]
impl HeavyDataBundle {
    fn new(
        data1: &HeavyData,
        data2: &HeavyData,
    ) -> Self {
        Self {
            data1: data1.clone(),
            data2: data2.clone(),
        }
    }
}
```

In this case, you can move the ownership to avoid copying.

```rust
#[savvy]
impl HeavyDataBundle {
    fn new(
        data1: HeavyData,
        data2: HeavyData,
    ) -> Self {
        Self { data1, data2 }
    }
}
```

Of course, this is an expert-only usage and is rarely needed. Again, you should
almost always use `&T` or `&mut T` instead of `T`. If you are really sure it
doesn't work well, you can use `T`.


## External pointer?

Under the hood, the `Person` struct is stored in `EXTPTRSXP`. But, you don't
need to care about how to deal with `EXTPTRSXP`. This is because it's stored in
a closure environemnt on creation and never exposed to the user. As it's
guaranteed on R's side that `self` is always a `EXTPTRSXP` of `Person`, Rust
code just restore a `Person` instance from the `EXTPTRSXP` without any checks.

```r
Person <- new.env(parent = emptyenv())
Person$new <- function() {
  .savvy_wrap_Person(.Call(Person_new__impl))
}

Person$say_hello <- function() {
.Call(Person_say_hello__impl)
}

.savvy_wrap_Person <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$set_name <- Person_set_name(ptr)
  e$name <- Person_name(ptr)

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
