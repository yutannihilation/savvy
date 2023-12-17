# savvy-ffi

Minimal FFI bindings for R's C API. This contains only a subset of APIs
sufficient for savvy framework. If you are looking for more complete one,
[libR-sys](https://crates.io/crates/libR-sys) is probably what you want.

Some more notable differences between libR-sys are:

* This is NOT a sys crate. Savvy-ffi is intended to be used within an R package,
  which compiles a staticlib from Rust code first and then links it to R. At the
  point of compilation by cargo, savvy-ffi is not yet linked, so this is fine.

* All definitions are written by hand, with some help of bindgen, into a single
  file. There's no automatic version switch or platform switch. If some switch
  is needed, it will be provided as a feature (e.g. `r_4_4_0`) and it's user's
  responsibility to set it properly.
