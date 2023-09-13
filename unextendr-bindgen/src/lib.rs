mod unextendr_fn;
mod unextendr_impl;
mod utils;

pub use unextendr_fn::{
    make_c_header_file, make_c_impl_file, make_r_impl_file, UnextendrFn, UnextendrFnArg,
    UnextendrFnType,
};

pub use unextendr_impl::UnextendrImpl;

pub use utils::extract_docs;
