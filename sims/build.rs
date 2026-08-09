use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_files(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files(&path, extension));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
                files.push(path);
            }
        }
    }
    files
}

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

    let cpp_dir = Path::new("cpp");
    println!("cargo:rerun-if-changed=cpp");

    // 1. Compile C++ code to WASM using C++26
    let mut build = cc::Build::new();
    build.cpp(true).std("c++26").cpp_link_stdlib(None).include("cpp");

    let cpp_files = collect_files(cpp_dir, "cpp");
    for file in &cpp_files {
        build.file(file);
    }
    build.compile("cpp_math");

    // 2. Generate Rust bindings using bindgen
    let mut builder = bindgen::builder().header("cpp/ffi.hpp").use_core();

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
