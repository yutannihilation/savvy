use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
        cc::Build::new()
            .file("src/unwind_protect_wrapper.c")
            .include(Path::new(d.as_str()))
            // Disable LTO for this TU. R's gcc-SAN builder injects `-flto=*`
            // via CFLAGS, which breaks linking against the Rust staticlib.
            // A trailing `-fno-lto` overrides it; `flag_if_supported` no-ops
            // on compilers that don't recognize the flag (e.g. MSVC).
            .flag_if_supported("-fno-lto")
            .compile("unwind_protect");
    } else {
        println!("cargo:warning=R_INCLUDE_DIR envvar should be provided.");
    }

    println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");
}
