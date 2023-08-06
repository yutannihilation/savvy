use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use syn::{parse_quote, FnArg::Typed, PatType};

mod unextendr_fn;

use unextendr_fn::make_c_header_file;

/// Generate C bindings and R bindings for a Rust library
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Path to a Rust file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut file = match File::open(args.file) {
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

    let unextendr_fns = ast
        .items
        .iter()
        .filter_map(unextendr_fn::parse_unextendr_fn)
        .collect::<Vec<unextendr_fn::UnextendrFn>>();

    println!("{}", make_c_header_file(&unextendr_fns));
}
