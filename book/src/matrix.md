# Handling Matrices And Arrays

Savvy doesn't provide a convenient way of converting matrices and arrays. You
have to do it by yourself. But, don't worry, it's probably not very difficult
thanks to the fact that major Rust matrix crates are column-majo, or at least
support column-major.

* [ndarray](https://crates.io/crates/ndarray): row-major is default (probably for compatibility with Python ndarray?), but it offers column-major as well
* [nalgebra](https://crates.io/crates/nalgebra): column-major
* [glam](https://crates.io/crates/glam) (and probably all other rust-gamedev crates): column-major, probably because GLSL is column-major

The example code can be found at <https://github.com/yutannihilation/savvy-matrix-examples/tree/master/src/rust/src>.

## R to Rust

### ndarray

By default, ndarray is row-major, but you can specify column-major by
[`f()`](https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#impl-ArrayBase%3CS%2C%20D%3E).
So, all you have to do is simply to extract the `dim` and pass it to ndarray.

```rust
use ndarray::Array;
use ndarray::ShapeBuilder;
use savvy::{r_println, savvy, RealSexp};

/// @export
#[savvy]
fn ndarray_input(x: RealSexp) -> savvy::Result<()> {
    // In R, dim is i32, so you need to convert it to usize first.
    let dim_i32 = x.get_dim().ok_or("no dimension found")?;
    let dim: Vec<usize> = dim_i32.iter().map(|i| *i as usize).collect();

    // f() changes the order from row-major (C-style convention) to column-major (Fortran-style convention).
    let a = Array::from_shape_vec(dim.f(), x.to_vec());

    r_println!("{a:?}");

    Ok(())
}
```

### nalgebra

nalgebra is column-major, so you can simply pass the `dim`.

```rust
use nalgebra::DMatrix;
use savvy::{r_println, savvy, RealSexp};

/// @export
#[savvy]
fn nalgebra_input(x: RealSexp) -> savvy::Result<()> {
    let dim = x.get_dim().ok_or("no dimension found")?;

    if dim.len() != 2 {
        return Err("Input must be matrix!".into());
    }

    let m = DMatrix::from_vec(dim[0] as _, dim[1] as _, x.to_vec());

    r_println!("{m:?}");

    Ok(())
}
```

### glam

glam is also column-major. In the case with glam, probably the dimension is
fixed (e.g. 3 x 3 in the following code). You can check the dimension is as
expected before passing it to the constructor of a matrix.

```rust
use glam::{dmat3, dvec3, DMat3};
use savvy::{r_println, savvy, OwnedRealSexp, RealSexp};

/// @export
#[savvy]
fn glam_input(x: RealSexp) -> savvy::Result<()> {
    let dim = x.get_dim().ok_or("no dimension found")?;

    if dim != [3, 3] {
        return Err("Input must be 3x3 matrix!".into());
    }

    // As we already check the dimension, this must not fail
    let x_array: &[f64; 9] = x.as_slice().try_into().unwrap();

    let m = DMat3::from_cols_array(x_array);

    r_println!("{m:?}");

    Ok(())
}
```

## Rust to R

The matrix libraries typically provides method to get the dimension and the
slice of underlying memory. You set the dimension by `set_dim()`.

```rust
/// @export
#[savvy]
fn nalgebra_output() -> savvy::Result<savvy::Sexp> {
    let m = DMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let shape = m.shape();
    let dim = &[shape.0, shape.1];

    let mut out = OwnedRealSexp::try_from(m.as_slice())?;
    out.set_dim(dim)?;

    out.into()
}
```
