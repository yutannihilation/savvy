use clap::{Parser, Subcommand};
use savvy_bindgen::generate_cargo_toml;
use savvy_bindgen::generate_example_lib_rs;
use savvy_bindgen::generate_gitignore;
use savvy_bindgen::generate_makevars;
use savvy_bindgen::generate_makevars_win;
use std::io::Write;
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

    /// Generate Makevars
    Makevars {
        /// package name
        crate_name: String,
    },

    /// Generate Makevars.win
    MakevarsWin {
        /// package name
        crate_name: String,
    },

    /// Generate .gitignore
    Gitignore {},

    /// Update wrappers in an R package
    Update {
        /// Path to the root of an R package
        r_pkg_dir: PathBuf,
    },

    /// Init savvy-powered Rust crate in an R package
    Init {
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
const PATH_SRC_DIR: &str = "src/rust/src";
const PATH_CARGO_TOML: &str = "src/rust/Cargo.toml";
const PATH_LIB_RS: &str = "src/rust/src/lib.rs";
const PATH_MAKEVARS: &str = "src/Makevars";
const PATH_MAKEVARS_WIN: &str = "src/Makevars.win";
const PATH_GITIGNORE: &str = "src/.gitignore";
const PATH_C_HEADER: &str = "src/rust/api.h";
const PATH_C_IMPL: &str = "src/init.c";
const PATH_R_IMPL: &str = "R/wrappers.R";

fn get_pkg_name(path: &Path) -> String {
    if !path.exists() {
        eprintln!("{} does not exist", path.to_string_lossy());
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("{} is not a directory", path.to_string_lossy());
        std::process::exit(1);
    }

    if let Some(pkg_name) = parse_description(&path.join(PATH_DESCRIPTION)) {
        pkg_name
    } else {
        eprintln!("{} is not an R package root", path.to_string_lossy());
        std::process::exit(4);
    }
}

fn write_file(path: &Path, contents: &str) {
    let path_str = path.to_string_lossy();
    println!("Writing {}", path_str);
    std::fs::write(path, contents).expect(&format!("Failed to write to {}", path_str));
}

fn update(path: &Path) {
    let pkg_name = get_pkg_name(path);

    let path_lib_rs = path.join(PATH_LIB_RS);
    println!("Parsing {}", path_lib_rs.to_string_lossy());
    let parsed_result = savvy_bindgen::parse_file(path_lib_rs.as_path());

    write_file(
        &path.join(PATH_C_HEADER),
        &generate_c_header_file(&parsed_result),
    );

    write_file(
        &path.join(PATH_C_IMPL),
        &generate_c_impl_file(&parsed_result, &pkg_name),
    );

    write_file(
        &path.join(PATH_R_IMPL),
        &generate_r_impl_file(&parsed_result, &pkg_name),
    );
}

fn init(path: &Path) {
    let pkg_name = get_pkg_name(path);

    if path.join("src").exists() {
        eprintln!("Aborting because `src` dir already exists.");
        return;
    }

    std::fs::create_dir_all(path.join(PATH_SRC_DIR)).expect("Failed to create src dir");

    write_file(&path.join(PATH_CARGO_TOML), &generate_cargo_toml(&pkg_name));
    write_file(&path.join(PATH_LIB_RS), &generate_example_lib_rs());
    write_file(&path.join(PATH_MAKEVARS), &generate_makevars(&pkg_name));
    write_file(
        &path.join(PATH_MAKEVARS_WIN),
        &generate_makevars_win(&pkg_name),
    );
    write_file(&path.join(PATH_GITIGNORE), &generate_gitignore());

    update(path);
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
        Commands::Makevars { crate_name } => {
            println!("{}", generate_makevars(&crate_name))
        }
        Commands::MakevarsWin { crate_name } => {
            println!("{}", generate_makevars_win(&crate_name))
        }
        Commands::Gitignore {} => {
            println!("{}", generate_gitignore())
        }
        Commands::Update { r_pkg_dir } => update(&r_pkg_dir),
        Commands::Init { r_pkg_dir } => init(&r_pkg_dir),
    }
}
