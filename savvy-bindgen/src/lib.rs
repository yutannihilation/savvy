mod gen;
mod parse_file;
mod savvy_fn;
mod savvy_impl;
mod utils;

pub use gen::c::{generate_c_header_file, generate_c_impl_file};
pub use gen::r::generate_r_impl_file;
pub use gen::static_files::{generate_gitignore, generate_makevars, generate_makevars_win};
pub use savvy_fn::{ParsedResult, SavvyFn, SavvyFnArg, SavvyFnType};
pub use savvy_impl::SavvyImpl;

pub use utils::extract_docs;

pub use parse_file::{parse_file, parse_savvy_fn, parse_savvy_impl, read_file};
