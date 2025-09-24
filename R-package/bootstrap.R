dep_dir <- "dep_crates"

# Tweak Cargo.toml
cargo_toml <- "src/rust/Cargo.toml"
lines <- readLines(cargo_toml)
writeLines(
  gsub("../../../", paste0("../", dep_dir, "/"), lines, fixed = TRUE),
  cargo_toml
)

dir.create(dep_dir)
file.copy(
  c(
    "src",
    "Cargo.toml",
    "build.rs",
    "savvy-macro",
    "savvy-bindgen",
    "savvy-ffi"
  ),
  dep_dir,
  recursive = TRUE
)
