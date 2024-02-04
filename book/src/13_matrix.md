# Handling Matrices And Arrays

Savvy doesn't provide a convenient way of converting matrices and arrays. You
have to do it by yourself. But, don't worry, it's probably not very difficult
thanks to the fact that major Rust matrix crates are column-majo, or at least
support column-major.

* ndarray: row-major is default (probably for compatibility with Python ndarray?), but it offers column-major as well
* nalgebra: column-major
* glam (and probably all other rust-gamedev crates): column-major, probably because GLSL is column-major

## ndarray

By default, ndarray is row-major, but you can specify column-major by
[`f()`](https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#impl-ArrayBase%3CS%2C%20D%3E).
So, all you have to do is simply to extract the `dim` and pass it to ndarray.

```rust
use ndarray::Array;
use ndarray::ShapeBuilder;
use savvy::r_println;
use savvy::{savvy, RealSexp};

#[savvy]
fn print_array(x: RealSexp) -> savvy::Result<()> {
    let dim = match x.get_dim() {
        Some(dim) => dim,
        None => {
            return Err("no dimension found!".into());
        }
    };

    // f() changes the order from row-major (C-style convention) to column-major (Fortran-style convention).
    let a = Array::from_shape_vec(dim.f(), x.to_vec());

    r_println!("{a:?}");

    Ok(())
}
```

## nalgebra

TBD

## glam

TBD