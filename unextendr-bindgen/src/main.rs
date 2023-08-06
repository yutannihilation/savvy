use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use syn::{parse_quote, FnArg::Typed, PatType};

mod unextendr_fn;

use unextendr_fn::make_c_header_file;
use unextendr_fn::make_c_impl_file;

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

fn parse_file(path: &PathBuf) -> Vec<unextendr_fn::UnextendrFn> {
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

    ast.items
        .iter()
        .filter_map(unextendr_fn::parse_unextendr_fn)
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap() {
        Commands::CHeader { file } => {
            let unextendr_fns = parse_file(&file);
            println!("{}", make_c_header_file(&unextendr_fns));
        }
        Commands::CImpl { file } => {
            let unextendr_fns = parse_file(&file);
            println!("{}", make_c_impl_file(&unextendr_fns));
        }
        Commands::RImpl { file } => {}
    }
}
