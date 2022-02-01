use std::process::Command;

fn main() {
    if !Command::new("make").args(["-C", "linuxdoom-1.10/"]).status().unwrap().success() {
        panic!("Makefile failed!");
    }
    println!("cargo:rerun-if-changed=linuxdoom-1.10/*.c");
    println!("cargo:rustc-link-search=linuxdoom-1.10/linux");
    println!("cargo:rustc-link-lib=static=linuxxdoom");
    println!("cargo:rustc-link-lib=dylib=Xext");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=nsl");
    println!("cargo:rustc-link-lib=dylib=m");
}