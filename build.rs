use std::env;
use std::path::PathBuf;

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
        // The input header we would like to generate
        // bindings for.
        .header("src/ctp/wrapper.hpp")
        /* // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) */
        .ignore_methods()
        .rustified_enum(".*")
        .blacklist_item("XTP_SIDE_TYPE")
        .blacklist_item("XTP_POSITION_EFFECT_TYPE")
        .blacklist_item("TXTPTradeTypeType")
        .blacklist_item("TXTPOrderTypeType")
        .blacklist_function("TraderSpiStub_Rust.*")
        .blacklist_function("QuoteSpiStub_Rust.*")
        /* .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        }) */
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}

fn main() {}
