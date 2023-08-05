use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use syn::{parse_quote, FnArg::Typed, PatType};

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

    for item in ast.items.iter() {
        match &item {
            syn::Item::Fn(func) => {
                // Generate bindings only when the function is marked by #[unextendr]
                if !func
                    .attrs
                    .iter()
                    .any(|attr| attr == &parse_quote!(#[unextendr]))
                {
                    continue;
                }

                let fn_name = func.sig.ident.to_string();
                let args =
                    func.sig
                        .inputs
                        .iter()
                        .map(|arg| {
                            if let Typed(PatType { pat, ty, .. }) = arg {
                                let pat_ident = match pat.as_ref() {
                                    syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                                    _ => panic!("Unsupported signature"),
                                };
                                let ty_ident = match ty.as_ref() {
                                    syn::Type::Path(path) => {
                                        let i = path.path.get_ident().unwrap();
                                        match i.to_string().as_str() {
                                            "IntegerSxp" | "RealSxp" | "LogicalSxp"
                                            | "StringSxp" => "SEXP",
                                            _ => panic!("Unsupported signature"),
                                        }
                                    }
                                    _ => panic!("Unsupported signature"),
                                };

                                format!("{ty_ident} {pat_ident}")
                            } else {
                                panic!("Unsupported signature")
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", ");
                println!("SEXP unextendr_{fn_name}({args});");
            }
            _ => continue,
        }
    }
}
