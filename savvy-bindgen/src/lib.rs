mod gen;
mod ir;
mod parse_file;
mod utils;

pub use gen::c::{generate_c_header_file, generate_c_impl_file};
pub use gen::r::generate_r_impl_file;
pub use gen::static_files::{
    generate_cargo_toml, generate_cleanup, generate_cleanup_win, generate_config_toml,
    generate_configure, generate_configure_win, generate_example_lib_rs, generate_gitignore,
    generate_makevars_in, generate_makevars_win_in, generate_win_def,
};
pub use ir::savvy_enum::SavvyEnum;
pub use ir::savvy_fn::{SavvyFn, SavvyFnArg, SavvyFnType};
pub use ir::savvy_impl::SavvyImpl;
pub use ir::savvy_struct::SavvyStruct;

pub use ir::{merge_parsed_results, MergedResult, ParsedResult};

pub use utils::extract_docs;

pub use parse_file::{generate_test_code, parse_file, read_file};
