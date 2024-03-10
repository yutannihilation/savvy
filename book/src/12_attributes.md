# Handling Attributes

You sometimes need to deal with attributes like `names` and `class`. Savvy
provides the following methods for getting and setting the value of the
attribute.


|           | Getter method | Setter method | Type         |
|:----------|:--------------|:--------------|:-------------|
| `names`   | `get_names()` |`set_names()`  | `Vec<&str>`  |
| `class`   | `get_class()` |`set_class()`  | `Vec<&str>`  |
| `dim`     | `get_dim()`   |`set_dim()`    | `&[i32]`     |
| arbitrary | `get_attrib()`|`set_attrib()` | `Sexp`       |

The getter methods return `Option<T>` because the object doesn't always have the
attribute. You can `match` the result like this:

```rust
#[savvy]
fn get_class_int(x: IntegerSexp) -> savvy::Result<savvy::Sexp> {
    match x.get_class() {
        Some(class) => class.try_into(),
        None => ().try_into(),
    }
}
```

The setter methods are available only for owned SEXPs. The return type is
`savvy::Result<()>` becuase the conversion from a Rust type to SEXP is fallible.

```rust
#[savvy]
fn set_class_int() -> savvy::Result<savvy::Sexp> {
    let mut x = OwnedIntegerSexp::new(1)?;

    x.set_class(&["foo", "bar"])?;

    x.into()
}
```

For attributes other than `names`, `class`, `dim`, you can use `get_attrib()`
and `set_attrib()`. Since an attribute can store arbitrary values, the type is
`Sexp`. In order to extract the underlying value, you can use `.into_typed()`
and `match`.

```rust
#[savvy]
fn print_attr_values_if_int(attr: &str, value: savvy::Sexp) -> savvy::Result<()>  {
    let attr_value = value.get_attrib(attr)?;
    match attr_value.into_typed() {
        TypedSexp::Integer(i) => r_println!("int {:?}", i.as_slice()]),
        _ => r_println("not int")
    }

    Ok(())
}
```

In order to set values, you can use `.into()` to convert from the owned SEXP to
a `savvy::Sexp`.

```rust
#[savvy]
fn set_attr_int(attr: &str) -> savvy::Result<savvy::Sexp> {
    let s: &[i32] = &[1, 2, 3];
    let attr_value: OwnedIntegerSexp = s.try_into()?;
    let mut out = OwnedIntegerSexp::new(1)?;

    out.set_attrib(attr, attr_value.into())?;

    out.into()
}
```
