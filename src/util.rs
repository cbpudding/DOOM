use std::{ffi::{CStr, CString}, mem, os::raw::{c_char, c_int}, slice};

#[export_name = "M_CheckParm"]
pub extern "C" fn check_parm(check: *mut c_char) -> c_int {
    let args = get_args();
    let target = unsafe { CString::from_raw(check).into_string().unwrap() };
    for i in 1..args.len() {
        let victim = unsafe { CStr::from_ptr(args[i]) };
        if target.eq_ignore_ascii_case(victim.to_str().unwrap()) {
            return i as c_int;
        }
    }
    mem::forget(target); // Don't free what we didn't allocate Rust!
    0
}

// Hopefully we'll be able to get rid of this thing when the code is a bit more Rust-y
pub fn get_args() -> &'static mut [*mut c_char] {
    unsafe {
        slice::from_raw_parts_mut(crate::global::MY_ARGV, crate::global::MY_ARGC as usize)
    }
}