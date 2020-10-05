#![allow(
dead_code,
unused_must_use,
unused_variables,
non_camel_case_types,
non_snake_case,
non_upper_case_globals,
unused_imports
)]

use std::env::var;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

// 衔接层  Rust ->  C -> C++

/// 遍历命令
fn build(target: &str, out_path: PathBuf) {
    let c = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    cc::Build::new()
        .file("src/ctp/src/ctp.cpp")
        .cpp(true)
        .warnings(false)
        .out_dir(get_interface_path(target))
        .include("/usr/include/")
        .flag(format!("-L {}/sdk_sources/ctp/linux", c.to_str().unwrap()).as_str())
        .compile("ctp");

    let bindings = bindgen::Builder::default()
        .header("src/ctp/wrapper.hpp")
        .derive_debug(true)
        .derive_default(true)
        .clang_arg("-I /usr/include/")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    let binding_output = bindings.to_string();
    let mut output_file = std::fs::File::create(out_path.as_path())
        .map_err(|e| format!("cannot create struct file, {}", e))
        .unwrap();
    output_file
        .write_all(binding_output.as_bytes())
        .map_err(|e| format!("cannot write struct file, {}", e));
}

fn main() {
    mkdir_home_path(vec!["ctp"]);

    let current_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // add c++ std library
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // link to ctp.a for covert level
    println!(
        "cargo:rustc-link-search={}",
        get_interface_path("ctp").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=ctp");

    // link to interface cpp
    println!("cargo:rustc-link-lib=dylib=stdc++");
    let lib_dir = format!("{}/sdk_sources/ctp/lib", current_dir.to_str().unwrap());
    let dylib_lib = format!("{}/sdk_sources/ctp/linux", current_dir.to_str().unwrap());

    // println!("cargo:rustc-flags=-C link-args=-Wl,-rpath,{}", dylib_lib);

    println!("cargo:rustc-link-search={}", dylib_lib);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");

    // println!("cargo:rerun-if-changed=src/wrapper.hpp");
    // println!("cargo:rerun-if-changed=src/bridge/bridge.hpp");
    // println!("cargo:rerun-if-changed=src/bridge/bridge.cpp");

    // output the bindings.rs and ctp.o to the .HFQ/ctp dir
    let out_path = get_interface_path("ctp").join("bindings.rs");
    if !out_path.exists() {
        build("ctp", out_path);
    }
}

fn get_interface_path(path: &str) -> PathBuf {
    let px = format!("{}/.HFQ/{}", var("HOME").unwrap(), path).to_string();
    let path_buffer = PathBuf::from(px);
    if !path_buffer.exists() {
        panic!("please mkdier interface dir fisrt");
    }
    path_buffer
}

/// mkdir at home path
fn mkdir_home_path(path: Vec<&str>) {
    let px = format!("{}/.HFQ", var("HOME").unwrap()).to_string();
    let home = Path::new(px.as_str());
    if !home.exists() {
        fs::create_dir(home);
    }
    for interface in path {
        let pw = home.clone().join(interface);
        if !pw.exists() {
            fs::create_dir(pw);
        }
    }
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}
