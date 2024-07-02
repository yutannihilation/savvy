# Type-specific Topics

You can use these types as an argument of a `#[savvy]` function.

| R type            | vector          | scalar      |
|:------------------|:----------------|:------------|
| integer           | `IntegerSexp`   | `i32`       |
| double            | `RealSexp`      | `f64`       |
| integer or double | `NumericSexp`   | `NumericScalar` |
| logical           | `LogicalSexp`   | `bool`      |
| raw               | `RawSexp`       | `u8`        |
| character         | `StringSexp`    | `&str`      |
| complex[^1]       | `ComplexSexp`   | `Complex64` |
| list              | `ListSexp`      | n/a         |
| (any)             | `Sexp`          | n/a         |

[^1]: Complex is optionally supported under feature flag `complex`

If you want to handle multiple types, you can cast an `Sexp` into a specific
type by `.into_typed()` and write `match` branches to deal with each type. This
is important when the interface returns `Sexp`. For example, `ListSexp` returns
`Sexp` because the list element can be any type. For more details about `List`,
please read [List](./list.md) section.

```rust
#[savvy]
fn print_list(x: ListSexp) -> savvy::Result<()> {
    for (k, v) in x.iter() {
        let content = match v.into_typed() {
            TypedSexp::Integer(x) => {
                format!(
                    "integer [{}]",
                    x.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")
                )
            }
            TypedSexp::Real(x) => {
                format!(
                    "double [{}]",
                    x.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", ")
                )
            }
            TypedSexp::Logical(x) => {
                format!(
                    "logical [{}]",
                    x.iter().map(|l| if l { "TRUE" } else { "FALSE" }).collect::<Vec<&str>>().join(", ")
                )
            }
            TypedSexp::String(x) => {
                format!(
                    "character [{}]",
                    x.iter().collect::<Vec<&str>>().join(", ")
                )
            }
            TypedSexp::List(_) => "list".to_string(),
            TypedSexp::Null(_) => "NULL".to_string(),
            _ => "other".to_string(),
        };

        let name = if k.is_empty() { "(no name)" } else { k };

        r_print!("{name}: {content}\n");
    }

    Ok(())
}
```

Likewise, `NumericSxep` also provides `into_typed()`. You can match it with
either `IntegerSexp` or `RealSexp` and apply an appropriate function.
Alternatively, you can rely on the type conversion that `NumericSexp` provides.
See more details in [the next section](./atomic_types.md).

```rust
#[savvy]
fn identity_num(x: NumericSexp) -> savvy::Result<savvy::Sexp> {
    match x.into_typed() {
        NumericTypedSexp::Integer(i) => identity_int(i),
        NumericTypedSexp::Real(r) => identity_real(r),
    }
}
```

