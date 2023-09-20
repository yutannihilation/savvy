use clap::{Parser, Subcommand};
use std::path::Path;
use std::path::PathBuf;

use savvy_bindgen::generate_c_header_file;
use savvy_bindgen::generate_c_impl_file;
use savvy_bindgen::generate_r_impl_file;

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

// Parse DESCRIPTION file and get the package name
fn parse_description(path: &Path) -> Option<String> {
    let content = savvy_bindgen::read_file(path);
    for line in content.lines() {
        if !line.starts_with("Package") {
            continue;
        }
        let mut s = line.split(':');
        s.next();
        if let Some(rhs) = s.next() {
            return Some(rhs.trim().to_string());
        }
    }

    None
}

const PATH_DESCRIPTION: &str = "DESCRIPTION";
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

    let pkg_name = parse_description(&path.join(PATH_DESCRIPTION));

    if pkg_name.is_none() {
        eprintln!("{} is not an R package root", path.to_string_lossy());
        std::process::exit(4);
    }
    let pkg_name = pkg_name.unwrap();

    let path_lib_rs = path.join(PATH_LIB_RS);
    println!("Parsing {}", path_lib_rs.to_string_lossy());
    let parsed_result = savvy_bindgen::parse_file(path_lib_rs.as_path());

    let path_c_header = path.join(PATH_C_HEADER);
    println!("Writing {}", path_c_header.to_string_lossy());
    std::fs::write(path_c_header, generate_c_header_file(&parsed_result)).unwrap();

    let path_c_impl = path.join(PATH_C_IMPL);
    println!("Writing {}", path_c_impl.to_string_lossy());
    std::fs::write(path_c_impl, generate_c_impl_file(&parsed_result, &pkg_name)).unwrap();

    let path_r_impl = path.join(PATH_R_IMPL);
    println!("Writing {}", path_r_impl.to_string_lossy());
    std::fs::write(path_r_impl, generate_r_impl_file(&parsed_result, &pkg_name)).unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CHeader { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!("{}", generate_c_header_file(&parsed_result));
        }
        Commands::CImpl { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!(
                "{}",
                generate_c_impl_file(&parsed_result, "%%PACKAGE_NAME%%")
            );
        }
        Commands::RImpl { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!(
                "{}",
                generate_r_impl_file(&parsed_result, "%%PACKAGE_NAME%%")
            );
        }
        Commands::Update { r_pkg_dir } => {
            update(r_pkg_dir.as_path());
        }
    }
}
