use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");

    // On macOS, the `cc` crate falls back to `xcrun --show-sdk-version` when
    // `MACOSX_DEPLOYMENT_TARGET` is unset, which can mismatch R's deployment
    // target and produce a linker warning. Default to 11.0 to match R's
    // typical baseline.
    #[cfg(target_os = "macos")]
    if std::env::var_os("MACOSX_DEPLOYMENT_TARGET").is_none() {
        // SAFETY: build.rs runs in a fresh single-threaded process spawned by Cargo.
        unsafe {
            std::env::set_var("MACOSX_DEPLOYMENT_TARGET", "11.0");
        }
    }

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
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
