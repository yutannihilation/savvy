use ndarray::Array;
use ndarray::ShapeBuilder;
use savvy::r_println;
use savvy::{savvy, RealSexp}; // Needed for .strides() method

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
