use std::path::Path;

fn main() {
    let r_include_dir =
        std::env::var("R_INCLUDE_DIR").expect("R_INCLUDE_DIR envvar must be provided.");

    cc::Build::new()
        .file("src/unwind_protect_wrapper.c")
        .include(Path::new(&r_include_dir))
        .compile("unwind_protect");

    println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
}
