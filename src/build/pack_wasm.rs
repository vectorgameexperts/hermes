use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;

use crate::build::hasher::compute_hash;

pub fn pack(current_dir: &str) {
    let current_dir_path = PathBuf::from(current_dir);
    let current_dir_name = current_dir_path
        .file_name()
        .unwrap()
        .to_str()
        .expect("Failed to convert current_dir to str");

    info!("Starting build processes...");
    // release build
    let wasm = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .arg("--target-dir")
        .arg("www-bindings")
        .output()
        .expect("Failed to execute cargo build [target wasm]");

    if wasm.status.success() {
        info!("build succeeded");
    } else {
        println!("build failed");
        println!("{}", String::from_utf8_lossy(&wasm.stderr));
        return;
    }

    // Get the wasm file path
    let wasm_file_path = format!(
        "www-bindings/wasm32-unknown-unknown/release/{}.wasm",
        current_dir_name
    );

    // Compute the hash
    let wasm_hash = compute_hash(&wasm_file_path);
    println!("Hash of wasm file: {}", wasm_hash);

    // rest of your code

    // Release build
    let bindgen = Command::new("wasm-bindgen")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("www")
        .arg(&wasm_file_path)
        .arg("--no-typescript")
        .output()
        .expect("Failed to execute bindgen");

    if bindgen.status.success() {
        info!("bindgen succeeded");
    } else {
        println!("bindgen failed");
        println!("{}", String::from_utf8_lossy(&bindgen.stderr));
    }
}
