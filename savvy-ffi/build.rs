use std::path::Path;

fn main() {
    let r_include_dir = std::env::var("R_INCLUDE_DIR");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // TODO: to pass the build of cargo-dist, this must be built without any errors.
    if let Ok(d) = r_include_dir {
        let mut build = cc::Build::new();
        build
            .file("src/backports/altrep_class.c")
            .include(Path::new(d.as_str()));

        // See ../build.rs for why this matters under `-flto` in CFLAGS.
        if target_os != "macos" {
            build.flag("-ffat-lto-objects");
        }

        build.compile("altrep_class");
    } else {
        println!("cargo:warning=R_INCLUDE_DIR envvar should be provided.");
    }

    println!("cargo:rerun-if-changed=src/backports/altrep_class.c");
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");
}
