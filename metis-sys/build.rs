use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    println!("cargo:rustc-link-lib=metis");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .generate()
        .unwrap_or_else(|err| {
            eprintln!("Failed to generate bindings to METIS: {err}");
            process::exit(1);
        });

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(&out_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to write METIS bindings to {:?}: {}",
            out_path.display(),
            err,
        );
        process::exit(1);
    });
}
