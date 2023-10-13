use std::process::Command;

use tracing::info;

//cargo install weasm-bindgen-cli

//wasm-bindgen --target web --out-dir bindings target/wasm32-unknown-unknown/debug/pluto.wasm --no-typescript
// cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort
pub fn pack() {
    info!("Starting build processes...");
    //release build
    let wasm = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .output()
        .expect("Failed to execute cargo build [target wasm]");
    // make new dir
    //wasm-bindgen --target web -- out-dir bindings <path to wasm>  --no-typescript

    if wasm.status.success() {
        info!("build succeeded");
    } else {
        println!("build failed");
        println!("{}", String::from_utf8_lossy(&wasm.stderr));
    }
    //release build
    //wasm-bindgen --target web --out-dir bindings
    //target/wasm32-unknown-unknown/debug/pluto.wasm --no-typescript
    // TODO: replace ".wasm with workspace name"
    let bindgen = Command::new("wasm-bindgen")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("www")
        .arg("../target/wasm32-unknown-unknown/release/client.wasm")
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
