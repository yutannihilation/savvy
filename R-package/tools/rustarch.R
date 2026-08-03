# The original file is in the hellorust pacakge by Jeroen Ooms, which is under the MIT license.
# cf. https://github.com/r-rust/hellorust/blob/master/tools/rustarch.R
arch <- if(grepl("aarch", R.version$platform)){
  "aarch64-pc-windows-gnullvm"
} else if(grepl("clang", Sys.getenv('R_COMPILED_BY'))){
  "x86_64-pc-windows-gnullvm"
} else {
  "x86_64-pc-windows-gnu"
}

cat(arch)
