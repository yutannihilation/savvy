use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
        // Strip any `-flto*` from CFLAGS before cc-rs reads it. R's gcc-SAN
        // builder injects `-flto=N` via CFLAGS; if the wrapper is compiled as
        // LTO bitcode, `unwind_protect_impl` is lost when Rust archives the
        // native static lib into the final staticlib, producing an undefined
        // symbol at package load time. Appending `-fno-lto` is not enough —
        // cc-rs' flag order doesn't reliably place user flags after CFLAGS.
        if let Ok(cflags) = std::env::var("CFLAGS") {
            let cleaned: String = cflags
                .split_whitespace()
                .filter(|f| !f.starts_with("-flto"))
                .collect::<Vec<_>>()
                .join(" ");
            // SAFETY: build.rs is single-threaded when this runs.
            unsafe { std::env::set_var("CFLAGS", cleaned); }
        }

        cc::Build::new()
            .file("src/unwind_protect_wrapper.c")
            .include(Path::new(d.as_str()))
            .compile("unwind_protect");
    } else {
        println!("cargo:warning=R_INCLUDE_DIR envvar should be provided.");
    }

    println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");
}
