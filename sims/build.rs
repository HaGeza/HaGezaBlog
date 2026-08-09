use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").unwrap_or_default();

    // 0. Use zig for wasm32; Not sure if this is useful
    unsafe {
        if target.contains("wasm32") {
            // Point cc to the Zig toolchain wrappers
            env::set_var("CC", "zig cc");
            env::set_var("CXX", "zig c++");
            env::set_var("AR", "zig ar");
        }
    }

    // 1. Compile C++ code to WASM using C++23
    let mut build = cc::Build::new();
    build
        .cpp(true) // Explicitly enable C++ mode
        .std("c++23") // Enable C++23 for constexpr / static_assert
        .cpp_link_stdlib(None)
        .file("cpp/tmp.cpp");

    build.compile("cpp_math");

    // 2. Generate Rust bindings using bindgen
    let mut builder = bindgen::builder().header("cpp/tmp.hpp").use_core();

    if target.contains("wasm32") {
        // Use a 32-bit target (i686) instead of 64-bit (x86_64)
        // This prevents 32-bit vs 64-bit pointer/size_t struct misalignment bugs
        builder = builder.clang_arg("--target=i686-unknown-linux-gnu");
    }

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    println!("cargo:rerun-if-changed=cpp");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
