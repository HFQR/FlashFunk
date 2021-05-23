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

#[cfg(not(target_os = "windows"))]
fn os_path() -> String {
    var("HOME").unwrap()
}

#[cfg(target_os = "windows")]
fn os_path() -> String {
    format!("{}{}", var("HOMEDRIVE").unwrap(), var("HOMEPATH").unwrap())
}

fn mkdir_path(path: &str) -> PathBuf {
    let os_path = os_path();
    println!("{}", os_path);
    let px = format!("{}/.HFQ", os_path);
    let home = Path::new(px.as_str());
    if !home.exists() {
        fs::create_dir(home);
    }
    let pw = home.join(path);
    if !pw.exists() {
        fs::create_dir(pw.clone());
    }
    PathBuf::from(pw)
}

#[cfg(not(target_os = "windows"))]
fn sdk_source_path(sdk: &str) -> (String, String, String, String) {
    let current_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let lib_dir = format!(
        "{}/flashfunk-level/sdk_sources/{}/lib",
        current_dir.to_str().unwrap(),
        sdk
    );
    let dll_dir = format!(
        "{}/flashfunk-level/sdk_sources/{}/linux",
        current_dir.to_str().unwrap(),
        sdk
    );
    let mut v1 = vec![];
    for entry in fs::read_dir(dll_dir.clone()).unwrap() {
        let filename = entry.unwrap().file_name().into_string().unwrap();
        v1.push(file_name(filename))
    }
    (
        lib_dir,
        dll_dir,
        v1.first().unwrap().clone(),
        v1.last().unwrap().clone(),
    )
}

fn file_name(name: String) -> String {
    if name.starts_with("lib") {
        let n: Vec<_> = name.split(".").collect();
        let name = n.first().unwrap().clone();
        name[3..].to_string()
    } else {
        let n: Vec<_> = name.split(".").collect();
        n.first().unwrap().to_string()
    }
}

#[cfg(target_os = "windows")]
fn sdk_source_path(sdk: &str) -> (String, String, String, String) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let current_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let lib_dir = format!(
        "{}\\flashfunk-level\\sdk_sources\\{}\\lib",
        current_dir.to_str().unwrap(),
        sdk
    );
    let dll_dir = format!(
        "{}\\flashfunk-level\\sdk_sources\\{}\\win",
        current_dir.to_str().unwrap(),
        sdk
    );
    let mut v = vec![];
    for entry in fs::read_dir(dll_dir.clone()).unwrap() {
        let file = entry.unwrap();
        let filepath = file.path();
        let filename = file.file_name().into_string().unwrap();
        let destination = format!("{}/{}", out_dir, filename);


        std::fs::copy(filepath, &destination).expect("failed to copy so to outdir");

        println!("cargo:resource={}", destination);

        v.push(file_name(filename))
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    (
        lib_dir,
        dll_dir,
        v.first().unwrap().clone(),
        v.last().unwrap().clone(),
    )
}

/// 遍历命令
fn build(target: &str) {
    let path = mkdir_path(target);
    let file_path = path.join("bindings.rs");

    let c = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    #[cfg(target_os = "windows")]
        cc::Build::new()
        .file(format!("src/{}/src/{}.cpp", target, target))
        .cpp(true)
        .warnings(false)
        .out_dir(path.clone())
        .include("/usr/include/")
        .flag(format!("-L {}/sdk_sources/{}/win", c.to_str().unwrap(), target).as_str())
        .compile(format!("{}", target).as_str());

    #[cfg(not(target_os = "windows"))]
        cc::Build::new()
        .file(format!("src/{}/src/{}.cpp", target, target))
        .cpp(true)
        .warnings(false)
        .out_dir(path.clone())
        .include("/usr/include/")
        .flag(format!("-L {}/sdk_sources/{}/linux", c.to_str().unwrap(), target).as_str())
        .compile(format!("{}", target).as_str());

    let bindings = bindgen::Builder::default()
        .header(format!("src/{}/wrapper.hpp", target))
        .derive_debug(true)
        .derive_default(true)
        .clang_arg("-I /usr/include/")
        .generate()
        .expect(format!("Unable to generate {} bindings", target).as_str());
    // write to file
    let bindings = bindings.to_string();
    let mut output_file = std::fs::File::create(file_path.as_path())
        .map_err(|e| format!("cannot create struct file, {}", e))
        .unwrap();
    output_file
        .write_all(bindings.as_bytes())
        .map_err(|e| format!("cannot write struct file, {}", e));

    // link to file
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    // println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    let (lib, dll, md, td) = sdk_source_path(target);

    // add  link search .lib  file path
    println!("cargo:rustc-link-search={}", lib);


    println!("cargo:rustc-link-lib=dylib={}", td);
    println!("cargo:rustc-link-lib=dylib={}", md);
}

fn main() {
    #[cfg(feature = "ctp")]
        build("ctp");

    #[cfg(feature = "ctp_mini")]
        build("ctp_mini");

    #[cfg(feature = "ess")]
        build("ess");

    #[cfg(feature = "rohon")]
        build("rohon");
}
