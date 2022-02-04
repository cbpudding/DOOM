use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=linuxdoom-1.10/wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("linuxdoom-1.10/wrapper.h")
        .clang_arg("-DLINUX")
        .clang_arg("-DNORMALUNIX")
        .clang_arg("-lXext")
        .clang_arg("-lX11")
        .clang_arg("-lnsl")
        .clang_arg("-lm")
        .clang_arg("-m32")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("doom.rs")).unwrap();
    if !Command::new("make").args(["-C", "linuxdoom-1.10/", "clean"]).status().unwrap().success() {
        panic!("Failed to clean C code!");
    }
    if !Command::new("make").args(["-C", "linuxdoom-1.10/"]).status().unwrap().success() {
        panic!("Failed to build C code!");
    }
    println!("cargo:rerun-if-changed=linuxdoom-1.10/Makefile");
    println!("cargo:rustc-link-search=linuxdoom-1.10/linux");
    println!("cargo:rustc-link-lib=static=linuxxdoom");
    println!("cargo:rustc-link-lib=dylib=Xext");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=nsl");
    println!("cargo:rustc-link-lib=dylib=m");
}