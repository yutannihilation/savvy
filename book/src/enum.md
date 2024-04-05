# Enum

Savvy supports **fieldless enum** to express the possible options for a
parameter. For example, if you define such an enum with `#[savvy]`,

```rust
/// @export
#[savvy]
enum LineType {
    Solid,
    Dashed,
    Dotted,
}
```

it will be available on R's side as this.

```r
LineType$Solid
LineType$Dashed
LineType$Dotted
```

You can use the enum type as the argument of such a function like this

```rust
/// @export
#[savvy]
fn plot_line(x: IntegerSexp, y: IntegerSexp, line_type: &LineType) -> savvy::Result<()> {
    match line_type {
        LineType::Solid => {
            ...
        },
        LineType::Dashed => {
            ...
        },
        LineType::Dotted => {
            ...
        },
    }
}
```

so that the users can use it instead of specifying it by an integer or a
character, which might be mistyped.

```r
plot_line(x, y, LineType$Solid)
```

Of course, you can archive the same thing with `i32` or `&str` as the input and
match the value. The difference is that enum is typo-proof. But, you might feel
it more handy to use a plain integer or character.

```rust
/// @export
#[savvy]
fn plot_line(x: IntegerSexp, y: IntegerSexp, line_type: &str) -> savvy::Result<()> {
    match line_type {
        "solid" => {
            ...
        },
        "dashed" => {
            ...
        },
        "dotted" => {
            ...
        },
        _ => {
            return Err("Unsupported line type!".into());
        }
    }
}
```

## Limitation

As noted above, savvy supports only fieldless enum for simplicity. If you want
to use an enum that contains some value, please wrap it with struct.

```rust
// You don't need to mark this with #[savvy]
enum AnimalEnum {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

/// @export
#[savvy]
struct Animal(AnimalEnum);
```

Also, savvy currently doesn't support discreminant. For example, this one won't
compile.

```rust
/// @export
#[savvy]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
}
```
