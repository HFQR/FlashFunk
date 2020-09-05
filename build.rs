#![allow(dead_code, unused_imports, unused_variables)]

use std::env;
use std::path::PathBuf;
use std::io::Write;

/// 遍历命令
fn build(target: &str) {
    println!("Ready to compile interface for ctp");
    cc::Build::new()
        .file("src/ctp/src/ctp.cpp")
        .cpp(true)
        .warnings(true)
        .flag("-std=c++11")
        .compile("bridge");
    let bindings = bindgen::Builder::default()
        .header("src/ctp/wrapper.hpp")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let env = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = env.join("bindings.rs");
    println!("{:?}", out_path);
    let binding_output = bindings.to_string();
    let mut output_file = std::fs::File::create(out_path.as_path()).map_err(|e| format!("cannot create struct file, {}", e)).unwrap();
    output_file.write_all(binding_output.as_bytes()).map_err(|e| format!("cannot write struct file, {}", e));
    println!("Generate successful")
}

fn main() {
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = format!("{}/sdk_sources/ctp/lib", current_dir);
    println!("{}", lib_dir);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    add_search_path(current_dir.clone());
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.cpp");
    build("ctp");
}


#[cfg(not(target_os = "windows"))]
fn add_search_path(main_path: String) {
    println!("cargo:rustc-flags=-L {}/sdk_sources/ctp/linux/", main_path);
    println!("cargo:rustc-link-search={}/sdk_sources/ctp/linux/", main_path);
}

#[cfg(target_os = "windows")]
fn add_search_path(main_path: String) {
    println!("cargo:rustc-flags=-L {}/sdk_sources/ctp/win/", main_path);
    println!("cargo:rustc-link-search={}/sdk_sources/ctp/win/", main_path);
}

#[cfg(target_arch = "x86_64")]
fn check_arch() {}

#[cfg(target_arch = "x86")]
fn check_arch() {
    unimplemented!("Not implemented for 32 bit system!");
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}