# List

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

## Read values from a list

### `names_iter()`

`names_iter()` returns an iterator of `&str`.

```rust
#[savvy]
fn print_list_names(x: ListSexp) -> savvy::Result<()> {
    for k in x.names_iter() {
        if k.is_empty() {
            r_println!("(no name)");
        } else {
            r_println!(k);
        }
        r_println!("");
    }

    Ok(())
}
```

```r
print_list_names(list(a = 1, 2, c = 3))
#> a
#> (no name)
#> c
```

### `values_iter()`

`values_iter()` returns an iterator of `Sexp` enum. You can convert `Sexp` to
`TypedSexp` by `.into_typed()` and then use `match` to extract the inner data.

```rust
#[savvy]
fn print_list_values_if_int(x: ListSexp) -> savvy::Result<()>  {
    for v in x.values_iter() {
        match v.into_typed() {
            TypedSexp::Integer(i) => r_println!("int {}\n", i.as_slice()[0]),
            _ => r_println("not int")
        }
    }

    Ok(())
}
```

```r
print_list_values_if_int(list(a = 1, b = 1L, c = "1"))
#> not int
#> int 1
#> not int
```

### `iter()`

If you want pairs of name and value, you can use `iter()`. This is basically a
`std::iter::Zip` of the two iterators explained above.

```rust
#[savvy]
fn print_list(x: ListSexp)  -> savvy::Result<()> {
    for (k, v) in x.iter() {
        // ...snip...
    }

    Ok(())
}
```

## Put values to a list

### `new()`

`OwnedListSexp`'s `new()` is different than other types; the second argument
(`named`) indicates whether the list is named or unnamed. If `false`, the list
doesn't have name and all operations on name like `set_name()` are simply
ignored.

### `set_name()`

`set_name()` simply sets a name at the specified position.

```rust
#[savvy]
fn list_with_no_values() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, true)?;

    out.set_name(0, "foo")?;
    out.set_name(1, "bar")?;

    out.into()
}
```
```r
list_with_no_values()
#> $foo
#> NULL
#> 
#> $bar
#> NULL
#> 
```

### `set_value()`

`set_value()` sets a value at the specified position. "Value" is an arbitrary
type that implmenents `Into<Sexp>` trait. Since all `{type}Sexp` types
implements it, you can simply pass it like below.

```rust
#[savvy]
fn list_with_no_names() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, false)?;

    let mut e1 = OwnedIntegerSexp::new(1)?;
    e1[0] = 100;
    
    let mut e2 = OwnedStringSexp::new(1)?;
    e2.set_elt(0, "cool")?;

    out.set_value(0, e1)?;
    out.set_value(1, e2)?;

    out.into()
}
```
```r
list_with_no_names()
#> [[1]]
#> [1] 100
#> 
#> [[2]]
#> [1] "cool"
#> 
```

### `set_name_and_value()`

`set_name_and_value()` is simply `set_name()` + `set_value()`. Probably this is
what you need in most of the cases.

```rust
#[savvy]
fn list_with_both() -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedListSexp::new(2, true)?;

    let mut e1 = OwnedIntegerSexp::new(1)?;
    e1[0] = 100;
    
    let mut e2 = OwnedStringSexp::new(1)?;
    e2.set_elt(0, "cool")?;

    out.set_name_and_value(0, "foo", e1)?;
    out.set_name_and_value(1, "bar", e2)?;

    out.into()
}
```
```r
list_with_both()
#> $foo
#> [1] 100
#> 
#> $bar
#> [1] "cool"
#> 
```
