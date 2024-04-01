mod utils;
use utils::*;

use std::collections::VecDeque;
use std::io::Write;

use clap::{Parser, Subcommand};

use std::path::Path;
use std::path::PathBuf;

use savvy_bindgen::{
    generate_c_header_file, generate_c_impl_file, generate_cargo_toml, generate_config_toml,
    generate_configure, generate_example_lib_rs, generate_gitignore, generate_makevars_in,
    generate_makevars_win, generate_r_impl_file, generate_win_def, ParsedResult,
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

impl PackageDescription {
    fn package_name_for_rust(&self) -> String {
        to_snake_case(&self.package_name)
    }

    fn package_name_for_r(&self) -> String {
        self.package_name.clone()
    }
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
        package_name: package_name_orig.to_string(),
        has_sysreq,
    }
}

const PATH_DESCRIPTION: &str = "DESCRIPTION";
const PATH_SRC_DIR: &str = "src/rust/src";
const PATH_DOT_CARGO_DIR: &str = "src/rust/.cargo";
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

fn update(path: &Path) {
    let pkg_metadata = get_pkg_metadata(path);

    let mut parsed: Vec<ParsedResult> = Vec::new();

    let lib_rs = path.join(PATH_SRC_DIR).join("lib.rs");
    let mut queue = VecDeque::from([lib_rs]);

    while !queue.is_empty() {
        let mut entry = queue.pop_front().unwrap();

        // there can be two patterns for a module named "bar"
        //
        // - foo/bar/mod.rs
        // - foo/bar.rs

        // if it's a directory, parse the mod.rs file first
        if entry.is_dir() {
            queue.push_back(entry.join("mod.rs"));
        }

        entry.set_extension("rs");

        if !entry.exists() || !entry.is_file() {
            continue;
        }

        println!("Parsing {}", entry.to_string_lossy());

        let result = savvy_bindgen::parse_file(&entry);

        // if the file has `mod` declarations, add the files to the queue.
        result
            .mod_dirs()
            .into_iter()
            .for_each(|x| queue.push_back(x));

        parsed.push(result);
    }

    write_file(
        &path.join(PATH_C_HEADER),
        &generate_c_header_file(parsed.as_slice()),
    );
    write_file(
        &path.join(PATH_C_IMPL),
        &generate_c_impl_file(parsed.as_slice(), &pkg_metadata.package_name_for_r()),
    );
    write_file(
        &path.join(PATH_R_IMPL),
        &generate_r_impl_file(parsed.as_slice(), &pkg_metadata.package_name_for_r()),
    );
}

fn init(path: &Path) {
    let pkg_metadata = get_pkg_metadata(path);

    if path.join("src").exists() {
        eprintln!("Aborting because `src` dir already exists.");
        return;
    }

    std::fs::create_dir_all(path.join(PATH_SRC_DIR)).expect("Failed to create src dir");
    std::fs::create_dir_all(path.join(PATH_DOT_CARGO_DIR)).expect("Failed to create .cargo dir");

    write_file(
        &path.join(PATH_CARGO_TOML),
        &generate_cargo_toml(&pkg_metadata.package_name_for_rust()),
    );
    write_file(&path.join(PATH_CONFIG_TOML), &generate_config_toml());
    write_file(&path.join(PATH_LIB_RS), &generate_example_lib_rs());
    write_file(
        &path.join(PATH_MAKEVARS_IN),
        &generate_makevars_in(&pkg_metadata.package_name_for_rust()),
    );
    write_file(&path.join(PATH_CONFIGURE), &generate_configure());
    set_executable(&path.join(PATH_CONFIGURE)); // This doesn't work on Windows!
    write_file(
        &path.join(format!(
            "src/{}-win.def",
            &pkg_metadata.package_name_for_r()
        )),
        &generate_win_def(&pkg_metadata.package_name_for_r()),
    );
    write_file(
        &path.join(PATH_MAKEVARS_WIN),
        &generate_makevars_win(&pkg_metadata.package_name_for_rust()),
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
        Commands::Update { r_pkg_dir } => update(&r_pkg_dir),
        Commands::Init { r_pkg_dir } => init(&r_pkg_dir),
    }
}
