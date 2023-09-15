use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");

    if let Ok(d) = r_include_dir {
        cc::Build::new()
            .file("src/unwind_protect_wrapper.c")
            .include(Path::new(d.as_str()))
            .compile("unwind_protect");
    } else {
        eprintln!("R_INCLUDE_DIR envvar must be provided.");
    }

    println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
}
