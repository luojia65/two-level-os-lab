use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    fs::File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    fs::File::create(out_dir.join("payload.x"))
        .unwrap()
        .write_all(include_bytes!("payload.x"))
        .unwrap();
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=payload.x");
    println!("cargo:rerun-if-changed=build.rs");
}
