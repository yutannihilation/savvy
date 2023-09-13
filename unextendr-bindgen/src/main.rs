use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::parse_quote;
use unextendr_fn::ParsedResult;

mod unextendr_fn;
mod unextendr_impl;
mod utils;

use unextendr_fn::make_c_header_file;
use unextendr_fn::make_c_impl_file;
use unextendr_fn::make_r_impl_file;
use unextendr_fn::UnextendrFn;
use unextendr_impl::UnextendrImpl;

/// Generate C bindings and R bindings for a Rust library
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
}

pub fn parse_unextendr_fn(item: &syn::Item) -> Option<UnextendrFn> {
    let func = match item {
        syn::Item::Fn(func) => func,
        _ => {
            return None;
        }
    };

    // Generate bindings only when the function is marked by #[unextendr]
    if func
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[unextendr]))
    {
        Some(UnextendrFn::from_fn(func))
    } else {
        None
    }
}

pub fn parse_unextendr_impl(item: &syn::Item) -> Vec<UnextendrFn> {
    let item_impl = match item {
        syn::Item::Impl(item_impl) => item_impl,
        _ => {
            return Vec::new();
        }
    };

    // Generate bindings only when the function is marked by #[unextendr]
    if item_impl
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[unextendr]))
    {
        UnextendrImpl::new(item_impl).fns
    } else {
        Vec::new()
    }
}

fn is_marked(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr == &parse_quote!(#[unextendr]))
}

fn parse_file(path: &PathBuf) -> ParsedResult {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to read the specified file");
            std::process::exit(1);
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
                    result.bare_fns.push(UnextendrFn::from_fn(&item_fn))
                }
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    result.impls.push(UnextendrImpl::new(&item_impl))
                }
            }
            _ => continue,
        };
    }

    result
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap() {
        Commands::CHeader { file } => {
            let parsed_result = parse_file(&file);
            println!("{}", make_c_header_file(&parsed_result));
        }
        Commands::CImpl { file } => {
            let parsed_result = parse_file(&file);
            println!("{}", make_c_impl_file(&parsed_result));
        }
        Commands::RImpl { file } => {
            let parsed_result = parse_file(&file);
            println!("{}", make_r_impl_file(&parsed_result));
        }
    }
}
