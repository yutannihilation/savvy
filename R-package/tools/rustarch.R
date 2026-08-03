# The original file is in the hellorust pacakge by Jeroen Ooms, which is under the MIT license.
# cf. https://github.com/r-rust/hellorust/blob/master/tools/rustarch.R
if(grepl("aarch", R.version$platform) || grepl("clang", Sys.getenv("R_COMPILED_BY"))){
  cat("aarch64-pc-windows-gnullvm")
} else {
  cat("x86_64-pc-windows-gnu")
}
