#![feature(c_variadic)]
#![feature(const_cstr_unchecked)]

use std::{env, ffi::CString, os::raw::c_int, ptr};

mod doom {
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/doom.rs"));
}

mod general;
mod global;
mod info;
mod misc;
mod native;
mod rendering;
mod types;
mod util;

fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(CString::new(arg).unwrap().into_raw());
    }
    args.push(ptr::null_mut());
    unsafe {
        global::MY_ARGC = (args.len() - 1) as c_int;
        global::MY_ARGV = args.as_mut_ptr();
        doom::D_DoomMain();
    }
}