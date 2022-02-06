use crate::{global::TICK_RATE, types::Ticcmd};
use std::{alloc::{self, Layout}, os::raw::{c_char, c_int}, process, ptr, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

#[export_name = "basetime"]
#[used]
pub static mut BASETIME: c_int = 0;

#[export_name = "emptycmd"]
#[used]
pub static mut EMPTY_CMD: Ticcmd = Ticcmd {
    forward_move: 0,
    side_move: 0,
    angle_turn: 0,
    consistency: 0,
    chat_char: 0,
    buttons: 0
};

#[export_name = "mb_used"]
#[used]
pub static mut MB_USED: c_int = 6;

#[export_name = "I_AllocLow"]
pub extern "C" fn allow_low(length: c_int) -> *mut u8 {
    let layout = Layout::from_size_align(length as usize, 1).unwrap();
    unsafe { alloc::alloc_zeroed(layout) }
}

#[export_name = "I_BaseTiccmd"]
pub extern "C" fn base_ticcmd() -> *mut Ticcmd {
    unsafe {
        ptr::addr_of_mut!(EMPTY_CMD)
    }
}

#[export_name = "I_BeginRead"]
pub extern "C" fn begin_read() {}

#[export_name = "I_EndRead"]
pub extern "C" fn end_read() {}

extern "C" {
    fn vsprintf(str: *mut c_char, format: *const c_char, args: ...) -> c_int;
}

#[export_name = "I_Error"]
pub unsafe extern "C" fn error(error: *const c_char, mut args: ...) {
    let mut buffer = [0u8; 1024]; // Error messages shouldn't be larger than 1K... right?
    vsprintf(buffer.as_mut_ptr() as *mut c_char, error, args.as_va_list());
    let message = String::from_utf8(Vec::from(buffer)).unwrap();
    if crate::doom::demorecording != 0 {
        crate::doom::G_CheckDemoStatus();
    }
    crate::doom::D_QuitNetGame();
    crate::doom::I_ShutdownGraphics();
    panic!("{}", message);
}

#[export_name = "I_GetHeapSize"]
pub extern "C" fn get_heap_size() -> c_int {
    unsafe { MB_USED as c_int * 1024 * 1024 }
}

#[export_name = "I_GetTime"]
pub extern "C" fn get_time() -> c_int {
    let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    unsafe {
        if BASETIME == 0 {
            BASETIME = current.as_secs() as c_int; // Year 2038 problem!
        }
        let result = (current.as_secs() as c_int - BASETIME) * TICK_RATE + current.subsec_micros() as c_int * TICK_RATE / 1_000_000;
        result
    }
}

#[export_name = "I_Init"]
pub extern "C" fn init() {
    unsafe {
        crate::doom::I_InitSound();
        // crate::doom::I_InitGraphics();
    }
}

#[export_name = "I_Quit"]
pub extern "C" fn quit() {
    unsafe {
        crate::doom::D_QuitNetGame();
        crate::doom::I_ShutdownSound();
        crate::doom::I_ShutdownMusic();
        crate::doom::M_SaveDefaults();
        crate::doom::I_ShutdownGraphics();
    }
    process::exit(0);
}

#[export_name = "I_Tactile"]
pub extern "C" fn tactile(on: c_int, off: c_int, total: c_int) {
    // on = off = total = 0;
}

#[export_name = "I_WaitVBL"]
pub extern "C" fn wait_vbl(count: c_int) {
    thread::sleep(Duration::from_micros(count as u64 * (1_000_000 / 70)));
}

#[export_name = "I_ZoneBase"]
pub extern "C" fn zone_base(size: *mut c_int) -> *mut u8 {
    unsafe {
        *size = MB_USED * 1024 * 1024;
        let layout = Layout::from_size_align(*size as usize, 1).unwrap();
        alloc::alloc(layout)
    }
}