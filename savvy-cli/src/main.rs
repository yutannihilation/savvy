mod utils;
use async_process::Stdio;
use savvy_bindgen::generate_test_code;
use savvy_bindgen::merge_parsed_results;
use savvy_bindgen::read_file;
use utils::*;

use std::collections::VecDeque;
use std::io::Write;

use clap::{Parser, Subcommand};

use std::path::Path;
use std::path::PathBuf;

use futures_lite::{io::BufReader, prelude::*};

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

    /// Run tests within an R session
    Test {
        /// Path to the lib.rs of the library (default: ./src/lib.rs)
        crate_dir: Option<PathBuf>,
        /// Path to the cache directory for placing a temporary R package for
        /// testing (default: <OS's cache dir>/savvy-cli-test/<crate name>)
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },

    /// Extract doctests and test modules
    ExtractTests {
        /// Path to the lib.rs of the library (default: ./src/lib.rs)
        crate_dir: Option<PathBuf>,
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

// Parse DESCRIPTION file and get the package name in a dirty way
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

// Parse Cargo.toml and get the crate name in a dirty way
fn parse_cargo_toml(path: &Path) -> String {
    let content = savvy_bindgen::read_file(path);

    let mut in_package_section = false;
    for line in content.lines() {
        if line.trim_start().starts_with('[') {
            in_package_section = line.trim() == "[package]";
            continue;
        }

        if !in_package_section {
            continue;
        }

        let mut s = line.split('=');
        if let Some(key) = s.next() {
            if key.trim() != "name" {
                continue;
            }
        }
        if let Some(value) = s.next() {
            return value.trim().trim_matches(['"', '\'']).to_string();
        }
    }

    eprintln!("Cargo.toml doesn't have package name!");
    std::process::exit(10);
}

const PATH_DESCRIPTION: &str = "DESCRIPTION";
const PATH_NAMESPACE: &str = "NAMESPACE";
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
const PATH_R_BUILDIGNORE: &str = ".Rbuildignore";

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

fn write_file(path: &Path, content: &str) {
    let mut opts = std::fs::OpenOptions::new();
    opts.create(true).write(true).truncate(true);
    write_file_inner(path, content, opts);
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

fn parse_crate(lib_rs: &Path, crate_name: &str) -> Vec<ParsedResult> {
    let mut parsed: Vec<ParsedResult> = Vec::new();

    if !lib_rs.exists() {
        eprintln!("{} doesn't exist!", lib_rs.to_string_lossy());
        std::process::exit(1);
    }

    // (file path, module path)
    let mut queue = VecDeque::from([(lib_rs.to_path_buf(), vec![crate_name.to_string()])]);

    while !queue.is_empty() {
        let (mut entry, mod_path) = queue.pop_front().unwrap();

        // there can be two patterns for a module named "bar"
        //
        // - foo/bar/mod.rs
        // - foo/bar.rs

        // if it's a directory, parse the mod.rs file
        if entry.is_dir() {
            entry.push("mod.rs");
        } else {
            entry.set_extension("rs");

            if !entry.exists() || !entry.is_file() {
                continue;
            }
        }

        eprintln!("Parsing {}", entry.to_string_lossy());

        let result = savvy_bindgen::parse_file(&entry, &mod_path);

        // if the file has `mod` declarations, add the files to the queue.
        result.child_mods.iter().for_each(|m| {
            let mut next_mod_path = mod_path.clone();
            next_mod_path.push(m.clone());
            queue.push_back((result.base_path.join(m), next_mod_path));
        });

        parsed.push(result);
    }
    parsed
}

fn tweak_r_buildignore(path: &Path) {
    let ignores = ["^src/rust/.cargo$", "^src/rust/target$"];
    let r_buildignore = path.join(PATH_R_BUILDIGNORE);
    if !r_buildignore.exists() {
        write_file(&r_buildignore, &format!("{}\n", ignores.join("\n")));
    } else {
        let content = read_file(&r_buildignore);
        let rest = ignores
            .into_iter()
            .filter(|&i| !content.contains(i))
            .collect::<Vec<&str>>()
            .join("\n");
        if !rest.is_empty() {
            append_file(&r_buildignore, &format!("\n{rest}\n"));
        }
    }
}

fn update(path: &Path) {
    let pkg_metadata = get_pkg_metadata(path);
    let lib_rs = path.join(PATH_SRC_DIR).join("lib.rs");
    let crate_name = parse_cargo_toml(&path.join(PATH_CARGO_TOML));

    let parsed = parse_crate(&lib_rs, &crate_name);

    let merged = merge_parsed_results(parsed);

    write_file(&path.join(PATH_C_HEADER), &generate_c_header_file(&merged));
    write_file(
        &path.join(PATH_C_IMPL),
        &generate_c_impl_file(&merged, &pkg_metadata.package_name_for_r()),
    );
    write_file(
        &path.join(PATH_R_IMPL),
        &generate_r_impl_file(&merged, &pkg_metadata.package_name_for_r()),
    );
    tweak_r_buildignore(path);
}

fn init(path: &Path, skip_update: bool) {
    let pkg_metadata = get_pkg_metadata(path);

    if path.join("src").exists() {
        eprintln!("Aborting because `src` dir already exists.");
        return;
    }

    std::fs::create_dir_all(path.join(PATH_SRC_DIR)).expect("Failed to create src dir");
    std::fs::create_dir_all(path.join(PATH_DOT_CARGO_DIR)).expect("Failed to create .cargo dir");

    write_file(
        &path.join(PATH_CARGO_TOML),
        &generate_cargo_toml(&pkg_metadata.package_name_for_rust(), r#"savvy = "*""#),
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

    if !skip_update {
        update(path);
    }
}

fn extract_tests(path: &Path, crate_name: &str) -> String {
    let parsed_results = parse_crate(path, crate_name);
    generate_test_code(&parsed_results)
}

fn create_empty_r_pkg(r_pkg_dir: &Path) -> std::io::Result<()> {
    let r_dir = r_pkg_dir.join("R");
    let namespace = r_pkg_dir.join(PATH_NAMESPACE);
    let description = r_pkg_dir.join(PATH_DESCRIPTION);

    // Create a minimal empty R package

    std::fs::create_dir_all(r_dir)?;

    std::fs::File::create(namespace)?;

    write_file(
        &description,
        "Package: SavvyRPkgForExtractedTests
Version: 0.0.0
Encoding: UTF-8
",
    );

    // Add files necessary for savvy

    if !r_pkg_dir.join("src").exists() {
        init(r_pkg_dir, true);
    }

    Ok(())
}

// The original wrapper expects the package is installed so that the symbols
// exist. But, when loaded externally, the symbol needs to be a character and
// `PACKAGE = ` specification.
fn tweak_wrapper_r(path: &Path) {
    let orig_content = savvy_bindgen::read_file(path);
    let mut content = orig_content.replace(".Call", ".Call_savvy_test");
    content.push_str(
        r#"
.Call_savvy_test <- function(symbol, ...) {
  symbol_string <- deparse(substitute(symbol))
  .Call(symbol_string, ..., PACKAGE = "SavvyRPkgForExtractedTests")
}
"#,
    );

    write_file(path, &content);
}

fn path_to_str(x: &Path) -> String {
    x.to_string_lossy()
        .trim_start_matches("\\\\?\\") // Tweak for Windows
        .replace('\\', "/")
}

async fn run_test(
    tests: String,
    crate_name: &str,
    crate_dir: &Path,
    tmp_r_pkg_dir: &Path,
) -> std::io::Result<()> {
    eprintln!(
        "\nUsing {} as the cache dir for testing...\n",
        tmp_r_pkg_dir.to_string_lossy()
    );
    create_empty_r_pkg(tmp_r_pkg_dir)?;

    // Inject the test code into lib.rs and add necessary dependencies into the crate.

    let pkg = get_pkg_metadata(tmp_r_pkg_dir);
    let rust_pkg_name = pkg.package_name_for_rust();
    let r_pkg_name = pkg.package_name_for_r();

    // Specify the crate to test as a path dependency.
    let crate_dir_abs = crate_dir.canonicalize()?;
    let crate_dir_abs = crate_dir_abs.to_string_lossy();
    #[cfg(windows)]
    let crate_dir_abs = if crate_dir_abs.starts_with(r#"\\?\"#) {
        crate_dir_abs.get(4..).unwrap().replace('\\', "/")
    } else {
        crate_dir_abs.replace('\\', "/")
    };
    let mut additional_deps = format!(r#"{crate_name} = {{ path = "{crate_dir_abs}" }}"#);
    if crate_name != "savvy" {
        additional_deps.push('\n');
        additional_deps.push_str(r#"savvy ="*""#);
    }
    write_file(
        &tmp_r_pkg_dir.join(PATH_CARGO_TOML),
        &generate_cargo_toml(
            &rust_pkg_name,
            // Cargo.toml is located at <crate dir>/.savvy/temporary-R-package-for-tests/src/rust/Cargo.toml
            &additional_deps,
        ),
    );
    // Since this can be within the workspace of a Rust package, clarify this is
    // not the part of it, otherwise the compilation will fail.
    append_file(&tmp_r_pkg_dir.join(PATH_CARGO_TOML), "[workspace]\n");

    write_file(&tmp_r_pkg_dir.join(PATH_LIB_RS), &tests);

    // Generate wrapper files
    update(tmp_r_pkg_dir);

    let wrapper_r = tmp_r_pkg_dir.join(PATH_R_IMPL);
    tweak_wrapper_r(&wrapper_r);

    let wrapper_r_str = path_to_str(&wrapper_r);
    let tmp_r_pkg_dir_str = path_to_str(tmp_r_pkg_dir);

    let tmp_r_script = tmp_r_pkg_dir.join("savvy-extracted-tests.R");
    write_file(
        &tmp_r_script,
        &format!(
            r###"
# Check if necessary package is installed
if (!"pkgbuild" %in% rownames(installed.packages())) {{
    stop("Please install the pkgbuild package to run tests\n", call. = FALSE)
}}

# Compile
pkgbuild::compile_dll("{tmp_r_pkg_dir_str}")

# Load the DLL
dll_file <- file.path("{tmp_r_pkg_dir_str}", "src", sprintf("%s%s", "{r_pkg_name}", .Platform$dynlib.ext))
dyn.load(dll_file)

# Load the wrapper R functions into an environment
e <- new.env()
source("{wrapper_r_str}", local = e)

# Run test functions
for (f in ls(e)) {{
    f <- get(f, e, inherits = FALSE)
    f()
}}

cat("test result: ok\n")
"###
        ),
    );

    eprintln!("\n--------------------------------------------\n");

    let mut cmd = async_process::Command::new("R")
        .args(["-q", "--no-echo", "-f", &tmp_r_script.to_string_lossy()])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut lines = BufReader::new(cmd.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        eprintln!("{}", line?);
    }

    let output = cmd.output().await?;

    if !output.status.success() {
        eprintln!("Test failed with status code {}", output.status);
        std::process::exit(1);
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update { r_pkg_dir } => update(&r_pkg_dir),
        Commands::Init { r_pkg_dir } => init(&r_pkg_dir, false),
        Commands::Test {
            crate_dir,
            cache_dir,
        } => {
            // Use the current dir as default
            let crate_dir = crate_dir.unwrap_or(".".into());
            let crate_name = parse_cargo_toml(&crate_dir.join("Cargo.toml"));

            // Use the OS's cache dir as default
            let cache_dir = match (cache_dir, dirs::cache_dir()) {
                (Some(p), _) => p,
                (None, Some(p)) => p.join("savvy-cli-test").join(&crate_name),
                (None, None) => {
                    eprintln!("No cache dir is available");
                    std::process::exit(1);
                }
            };

            let tests = extract_tests(&crate_dir.join("src/lib.rs"), &crate_name);
            match futures_lite::future::block_on(run_test(
                tests,
                &crate_name,
                &crate_dir,
                &cache_dir,
            )) {
                Ok(_) => {}
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        eprintln!("`R` is not found on PATH. Please add R to PATH.");
                        std::process::exit(1);
                    }
                    _ => {
                        panic!("{e:#?}");
                    }
                },
            }
        }
        Commands::ExtractTests { crate_dir } => {
            let dir = crate_dir.unwrap_or(".".into());
            let crate_name = parse_cargo_toml(&dir.join("Cargo.toml"));
            let tests = extract_tests(&dir.join("src/lib.rs"), &crate_name);
            println!("{tests}");
        }
    }
}
