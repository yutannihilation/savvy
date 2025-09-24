# Tweak Cargo.toml
cargo_toml <- "src/rust/Cargo.toml"
lines <- readLines(cargo_toml)
writeLines(
  gsub("../../../", "../dep_crates/", lines, fixed = TRUE),
  cargo_toml
)

dir.create("src/dep_crates/")
file.copy(
  c(
    "../src",
    "../Cargo.toml",
    "../build.rs",
    "../savvy-macro",
    "../savvy-bindgen",
    "../savvy-ffi"
  ),
  "src/dep_crates/",
  recursive = TRUE
)
