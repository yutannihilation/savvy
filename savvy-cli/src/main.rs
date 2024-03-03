mod utils;
use utils::*;

use std::io::Write;

use clap::{Parser, Subcommand};

use std::path::Path;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

use savvy_bindgen::{
    generate_c_header_file, generate_c_impl_file, generate_cargo_toml, generate_config_toml,
    generate_configure, generate_example_lib_rs, generate_gitignore, generate_makevars_in,
    generate_makevars_win, generate_r_impl_file, ParsedResult,
};

/// Generate C bindings and R bindings for a Rust library
#[derive(Parser, Debug)]
#[command(about, version, long_about = None)]
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

    /// Generate Makevars.in
    MakevarsIn {
        /// package name
        crate_name: String,
    },

    /// Generate configure
    Configure {},

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

struct PackageDescription {
    package_name: String,
    has_sysreq: bool,
}

// Parse DESCRIPTION file and get the package name
fn parse_description(path: &Path) -> PackageDescription {
    let content = savvy_bindgen::read_file(path);
    let mut package_name_orig = "";
    let mut has_sysreq = false;

    for line in content.lines() {
        if line.starts_with("Package") {
            let mut s = line.split(':');
            s.next();
            if let Some(rhs) = s.next() {
                package_name_orig = rhs.trim();
            }
        }

        if line.starts_with("SystemRequirements") {
            has_sysreq = true;
        }
    }

    if package_name_orig.is_empty() {
        eprintln!("{} is not an R package root", path.to_string_lossy());
        std::process::exit(4);
    }

    PackageDescription {
        package_name: to_snake_case(package_name_orig),
        has_sysreq,
    }
}

const PATH_DESCRIPTION: &str = "DESCRIPTION";
const PATH_SRC_DIR: &str = "src/rust/src";
const PATH_CARGO_TOML: &str = "src/rust/Cargo.toml";
const PATH_CONFIG_TOML: &str = "src/rust/.cargo/config.toml";
const PATH_LIB_RS: &str = "src/rust/src/lib.rs";
const PATH_MAKEVARS_IN: &str = "src/Makevars.in";
const PATH_CONFIGURE: &str = "configure";
const PATH_MAKEVARS_WIN: &str = "src/Makevars.win";
const PATH_GITIGNORE: &str = "src/.gitignore";
const PATH_C_HEADER: &str = "src/rust/api.h";
const PATH_C_IMPL: &str = "src/init.c";
const PATH_R_IMPL: &str = "R/wrappers.R";

fn get_pkg_metadata(path: &Path) -> PackageDescription {
    if !path.exists() {
        eprintln!("{} does not exist", path.to_string_lossy());
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("{} is not a directory", path.to_string_lossy());
        std::process::exit(1);
    }

    parse_description(&path.join(PATH_DESCRIPTION))
}

fn write_file_inner(path: &Path, contents: &str, open_opts: std::fs::OpenOptions) {
    let path_str = path.to_string_lossy();
    println!("Writing {}", path_str);

    open_opts
        .open(path)
        .unwrap_or_else(|_| panic!("Failed to open {}", path_str))
        .write_all(contents.as_bytes())
        .unwrap_or_else(|_| panic!("Failed to write {}", path_str));
}

fn write_file(path: &Path, contents: &str) {
    let mut opts = std::fs::OpenOptions::new();
    opts.create(true).write(true).truncate(true);
    write_file_inner(path, contents, opts);
}

fn append_file(path: &Path, contents: &str) {
    let mut opts = std::fs::OpenOptions::new();
    opts.append(true);
    write_file_inner(path, contents, opts);
}

// TODO: how can this be done on Windows?
#[cfg(unix)]
fn set_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let path_str = path.to_string_lossy();
    println!("Setting {} as executable", path_str);

    let mut perm = std::fs::metadata(path).unwrap().permissions();
    perm.set_mode(0o755);
    std::fs::set_permissions(path, perm).unwrap();
}

#[cfg(not(unix))]
fn set_executable(path: &Path) {
    let path_str = path.to_string_lossy();
    eprintln!(
        "
### Warning ###################################################################

On Windows, please manually run `git update-index --add --chmod=+x {path_str}`
to set the configure script as executable

###############################################################################
"
    );
}

fn get_rust_file(x: walkdir::Result<DirEntry>) -> Option<DirEntry> {
    if let Ok(entry) = x {
        if entry.file_name().to_string_lossy().ends_with(".rs") {
            Some(entry)
        } else {
            None
        }
    } else {
        None
    }
}

fn update(path: &Path) {
    let pkg_metadata = get_pkg_metadata(path);
    let mut parsed: Vec<ParsedResult> = Vec::new();

    for e in WalkDir::new(path.join(PATH_SRC_DIR))
        .into_iter()
        .filter_map(get_rust_file)
    {
        println!("Parsing {}", e.path().to_string_lossy());
        parsed.push(savvy_bindgen::parse_file(e.path()));
    }

    write_file(
        &path.join(PATH_C_HEADER),
        &generate_c_header_file(parsed.as_slice()),
    );
    write_file(
        &path.join(PATH_C_IMPL),
        &generate_c_impl_file(parsed.as_slice(), &pkg_metadata.package_name),
    );
    write_file(
        &path.join(PATH_R_IMPL),
        &generate_r_impl_file(parsed.as_slice(), &pkg_metadata.package_name),
    );
}

fn init(path: &Path) {
    let pkg_metadata = get_pkg_metadata(path);

    if path.join("src").exists() {
        eprintln!("Aborting because `src` dir already exists.");
        return;
    }

    std::fs::create_dir_all(path.join(PATH_SRC_DIR).join(".cargo"))
        .expect("Failed to create src dir");

    write_file(
        &path.join(PATH_CARGO_TOML),
        &generate_cargo_toml(&pkg_metadata.package_name),
    );
    write_file(&path.join(PATH_CONFIG_TOML), &generate_config_toml());
    write_file(&path.join(PATH_LIB_RS), &generate_example_lib_rs());
    write_file(
        &path.join(PATH_MAKEVARS_IN),
        &generate_makevars_in(&pkg_metadata.package_name),
    );
    write_file(&path.join(PATH_CONFIGURE), &generate_configure());
    set_executable(&path.join(PATH_CONFIGURE));
    write_file(
        &path.join(PATH_MAKEVARS_WIN),
        &generate_makevars_win(&pkg_metadata.package_name),
    );
    write_file(&path.join(PATH_GITIGNORE), &generate_gitignore());

    if pkg_metadata.has_sysreq {
        eprintln!(
            "
### Warning ###################################################################

\"SystemRequirements\" field already exists.
Please make sure \"Cargo (Rust's package manager), rustc\" is included.

###############################################################################

"
        )
    } else {
        append_file(
            &path.join(PATH_DESCRIPTION),
            // cf. https://cran.r-project.org/web/packages/using_rust.html
            "SystemRequirements: Cargo (Rust's package manager), rustc\n",
        );
    }

    update(path);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CHeader { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!("{}", generate_c_header_file(&[parsed_result]));
        }
        Commands::CImpl { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!(
                "{}",
                generate_c_impl_file(&[parsed_result], "%%PACKAGE_NAME%%")
            );
        }
        Commands::RImpl { file } => {
            let parsed_result = savvy_bindgen::parse_file(file.as_path());
            println!(
                "{}",
                generate_r_impl_file(&[parsed_result], "%%PACKAGE_NAME%%")
            );
        }
        Commands::MakevarsIn { crate_name } => {
            println!("{}", generate_makevars_in(&crate_name))
        }
        Commands::Configure {} => {
            println!("{}", generate_configure())
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
