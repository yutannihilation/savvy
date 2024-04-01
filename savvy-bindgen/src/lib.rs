mod gen;
mod parse_file;
mod savvy_fn;
mod savvy_impl;
mod savvy_struct;
mod utils;

pub use gen::c::{generate_c_header_file, generate_c_impl_file};
pub use gen::r::generate_r_impl_file;
pub use gen::static_files::{
    generate_cargo_toml, generate_config_toml, generate_configure, generate_example_lib_rs,
    generate_gitignore, generate_makevars_in, generate_makevars_win, generate_win_def,
};
pub use savvy_fn::{ParsedResult, SavvyFn, SavvyFnArg, SavvyFnType};
pub use savvy_impl::SavvyImpl;
pub use savvy_struct::SavvyStruct;

pub use utils::extract_docs;

pub use parse_file::{parse_file, read_file};
