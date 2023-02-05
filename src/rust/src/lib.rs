use extendr_api::prelude::*;

/// Return a static string.
///
/// @export
#[extendr(use_try_from = true)]
fn static_string() -> &'static str {
    "Hello world!"
}

/// Return a dynamic string.
///
/// @export
#[extendr(use_try_from = true)]
fn string(input: &str) -> String {
    input.to_string()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod unextendr;
    fn static_string;
    fn string;
}
