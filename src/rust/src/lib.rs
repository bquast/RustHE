use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Add two interegers and return the result to R.
/// @export
#[extendr]
fn add(x: i32, y: i32) -> i32 {
    x + y
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod RustHE;
    fn hello_world;
    fn add;
}


