use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
        cc::Build::new()
            .file("src/unwind_protect_wrapper.c")
            .include(Path::new(d.as_str()))
            .compile("unwind_protect");

        println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
    } else {
        cc::Build::new()
            .file("src/unwind_protect_fake.c")
            .compile("unwind_protect");
        println!("cargo:warning=R_INCLUDE_DIR envvar should be provided.");

        println!("cargo:rerun-if-changed=src/unwind_protect_fake.c");
    }
}
