use clap::{Parser, Subcommand};
use savvy_fn::ParsedResult;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use syn::parse_quote;

mod savvy_fn;
mod savvy_impl;
mod utils;

use savvy_fn::make_c_header_file;
use savvy_fn::make_c_impl_file;
use savvy_fn::make_r_impl_file;
use savvy_fn::SavvyFn;
use savvy_impl::SavvyImpl;

/// Generate C bindings and R bindings for a Rust library
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate C header file
    CHeader {
        /// Path to a Rust file
        file: PathBuf,
    },

    /// Generate C implementation for init.c
    CImpl {
        /// Path to a Rust file
        file: PathBuf,
    },

    /// Generate R wrapper functions
    RImpl {
        /// Path to a Rust file
        file: PathBuf,
    },

    /// Update wrappers in an R package
    Update {
        /// Path to the root of an R package
        r_pkg_dir: PathBuf,
    },
}

pub fn parse_savvy_fn(item: &syn::Item) -> Option<SavvyFn> {
    let func = match item {
        syn::Item::Fn(func) => func,
        _ => {
            return None;
        }
    };

    // Generate bindings only when the function is marked by #[savvy]
    if func
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[savvy]))
    {
        Some(SavvyFn::from_fn(func))
    } else {
        None
    }
}

pub fn parse_savvy_impl(item: &syn::Item) -> Vec<SavvyFn> {
    let item_impl = match item {
        syn::Item::Impl(item_impl) => item_impl,
        _ => {
            return Vec::new();
        }
    };

    // Generate bindings only when the function is marked by #[savvy]
    if item_impl
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[savvy]))
    {
        SavvyImpl::new(item_impl).fns
    } else {
        Vec::new()
    }
}

fn is_marked(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr == &parse_quote!(#[savvy]))
}

fn parse_file(path: &Path) -> ParsedResult {
    if !path.exists() {
        eprintln!("{} does not exist", path.to_string_lossy());
        std::process::exit(1);
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to read the specified file");
            std::process::exit(2);
        }
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        eprintln!("Failed to read the specified file");
        std::process::exit(2);
    };

    let ast = match syn::parse_str::<syn::File>(&content) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("Failed to parse the specified file");
            std::process::exit(3);
        }
    };

    let mut result = ParsedResult {
        bare_fns: Vec::new(),
        impls: Vec::new(),
    };

    for item in ast.items {
        match item {
            syn::Item::Fn(item_fn) => {
                if is_marked(item_fn.attrs.as_slice()) {
                    result.bare_fns.push(SavvyFn::from_fn(&item_fn))
                }
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    result.impls.push(SavvyImpl::new(&item_impl))
                }
            }
            _ => continue,
        };
    }

    result
}

const PATH_LIB_RS: &str = "src/rust/src/lib.rs";
const PATH_C_HEADER: &str = "src/rust/api.h";
const PATH_C_IMPL: &str = "src/init.c";
const PATH_R_IMPL: &str = "R/wrappers.R";

fn update(path: &Path) {
    if !path.exists() {
        eprintln!("{} does not exist", path.to_string_lossy());
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("{} is not a directory", path.to_string_lossy());
        std::process::exit(1);
    }

    if !path.join("DESCRIPTION").exists() {
        eprintln!("{} is not an R package root", path.to_string_lossy());
        std::process::exit(4);
    }

    let path_lib_rs = path.join(PATH_LIB_RS);
    let parsed_result = parse_file(path_lib_rs.as_path());
    std::fs::write(path.join(PATH_C_HEADER), make_c_header_file(&parsed_result)).unwrap();
    std::fs::write(path.join(PATH_C_IMPL), make_c_impl_file(&parsed_result)).unwrap();
    std::fs::write(path.join(PATH_R_IMPL), make_r_impl_file(&parsed_result)).unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CHeader { file } => {
            let parsed_result = parse_file(file.as_path());
            println!("{}", make_c_header_file(&parsed_result));
        }
        Commands::CImpl { file } => {
            let parsed_result = parse_file(file.as_path());
            println!("{}", make_c_impl_file(&parsed_result));
        }
        Commands::RImpl { file } => {
            let parsed_result = parse_file(file.as_path());
            println!("{}", make_r_impl_file(&parsed_result));
        }
        Commands::Update { r_pkg_dir } => {
            update(r_pkg_dir.as_path());
        }
    }
}
