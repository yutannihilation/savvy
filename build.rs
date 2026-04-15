use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
        let mut build = cc::Build::new();
        build
            .file("src/unwind_protect_wrapper.c")
            .include(Path::new(d.as_str()));

        // When R's CFLAGS inject `-flto=N` (e.g. gcc-SAN builder), gcc emits
        // slim LTO objects whose symbols don't land in the archive index
        // after Rust `ar`s them into the final staticlib, causing
        // `unwind_protect_impl` to go missing at package load. Fat LTO
        // objects carry both bitcode and real ELF symbols, which keeps the
        // archive index correct. Skipped on macOS because clang doesn't
        // recognize the flag (and cc-rs' `flag_if_supported` probe would
        // emit a `flag_check.c` that trips a CRAN line-endings NOTE).
        if target_os != "macos" {
            build.flag("-ffat-lto-objects");
        }

        build.compile("unwind_protect");
    } else {
        println!("cargo:warning=R_INCLUDE_DIR envvar should be provided.");
    }

    println!("cargo:rerun-if-changed=src/unwind_protect_wrapper.c");
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");
}
