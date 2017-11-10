extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("../../../backend-src/backend.h")
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("backend.rs")).expect(
        "Failed to write bindings",
    );

    let lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../build/native");
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=backend");
}
