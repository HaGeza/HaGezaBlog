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
    let cpp_dir = Path::new("cpp");

    // 1. Compile C++ files into a static library ("libcpp_math.a")
    let mut build = cc::Build::new();
    build.cpp(true).std("c++26").cpp_link_stdlib(None).include("cpp");

    let cpp_files = collect_files(cpp_dir, "cpp");
    for file in &cpp_files {
        build.file(file);
    }
    build.compile("cpp_math");

    // 2. Re-run build script if any C++ file or build.rs changes
    println!("cargo:rerun-if-changed=cpp");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
