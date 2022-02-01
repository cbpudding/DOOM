use std::{env, ffi::CString, os::raw::{c_char, c_int}, ptr};

extern "C" {
    pub static mut myargc: c_int;
    pub static mut myargv: *mut *mut c_char;
    pub fn D_DoomMain();
}

fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(CString::new(arg).unwrap().into_raw());
    }
    args.push(ptr::null_mut());
    unsafe {
        myargc = (args.len() - 1) as c_int;
        myargv = args.as_mut_ptr();
        D_DoomMain();
    }
}