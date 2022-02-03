use std::{env, ffi::CString, os::raw::c_int, ptr};

mod doom {
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/doom.rs"));
}

mod doomstat;
mod types;

fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(CString::new(arg).unwrap().into_raw());
    }
    args.push(ptr::null_mut());
    unsafe {
        doom::myargc = (args.len() - 1) as c_int;
        doom::myargv = args.as_mut_ptr();
        doom::D_DoomMain();
    }
}