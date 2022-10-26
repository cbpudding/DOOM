use crate::{doom, misc::random};
use std::{
    mem,
    os::raw::{c_int, c_short},
    ptr,
    slice,
};

static mut GO: bool = false;

static mut WIPE_SCR_START: *mut u8 = ptr::null_mut();
static mut WIPE_SCR_END: *mut u8 = ptr::null_mut();
static mut WIPE_SCR: *mut u8 = ptr::null_mut();

#[export_name = "wipe_shittyColMajorXform"]
pub extern "C" fn shitty_col_major_x_form(array: *mut c_short, width: c_int, height: c_int) {
    let array_slice = unsafe { slice::from_raw_parts_mut(array, (width * height) as usize) };
    let mut dest = vec![0i16; (width * height) as _].into_boxed_slice();
    for y in 0..height {
        for x in 0..width {
            dest[(x * height + y) as usize] = array_slice[(y * width + x) as usize];
        }
    }
    array_slice.clone_from_slice(dest.as_ref());
}

#[export_name = "wipe_initColorXForm"]
// TODO: The "ticks" argument isn't used in the following code. When changing the function signature this should be fixed. ~Alex
pub extern "C" fn init_color_x_form(width: c_int, height: c_int, _ticks: c_int) -> c_int {
    let wipe_scr_start = unsafe { slice::from_raw_parts_mut(WIPE_SCR_START, (width * height) as _) };
    let wipe_scr = unsafe { slice::from_raw_parts_mut(WIPE_SCR, (width * height) as _) };
    wipe_scr.clone_from_slice(wipe_scr_start.as_ref());
    // TODO: This function always returns 0 so it shouldn't return anything at all! ~Alex
    0
}

#[export_name = "wipe_doColorXForm"]
pub extern "C" fn do_color_x_form(width: c_int, height: c_int, ticks: c_int) -> c_int {
    let mut changed = false;
    let mut w = unsafe { WIPE_SCR };
    let mut e = unsafe { WIPE_SCR_END };
    let mut newval = 0;
    while w != unsafe { WIPE_SCR.offset((width * height) as _) } {
        if unsafe { *w != *e } {
            if unsafe { *w > *e } {
                newval = unsafe { *w as c_int } - ticks;
                if newval < unsafe { *e as _ } {
                    unsafe { *w = *e };
                } else {
                    unsafe { *w = newval as _ };
                }
                changed = true;
            } else if unsafe { *w < *e } {
                newval = unsafe { *w as c_int } + ticks;
                if newval > unsafe { *e as _ } {
                    unsafe { *w = *e };
                } else {
                    unsafe { *w = newval as _ };
                }
                changed = true;
            }
        }
        w = unsafe { w.offset(1) };
        e = unsafe { e.offset(1) };
    }
    !changed as _
}

#[export_name = "wipe_exitColorXForm"]
// TODO: This function does *nothing* and probably should be removed.
pub extern "C" fn exit_color_x_form(_width: c_int, _height: c_int, _ticks: c_int) -> c_int { 0 }

static mut Y: *mut c_int = ptr::null_mut();

#[export_name = "wipe_initMelt"]
pub extern "C" fn init_melt(width: c_int, height: c_int, _ticks: c_int) -> c_int {
    let wipe_scr_start = unsafe { slice::from_raw_parts_mut(WIPE_SCR_START, (width * height) as _) };
    let wipe_scr = unsafe { slice::from_raw_parts_mut(WIPE_SCR, (width * height) as _) };
    wipe_scr.clone_from_slice(wipe_scr_start);
    shitty_col_major_x_form(unsafe { WIPE_SCR_START as _ }, width / 2, height);
    shitty_col_major_x_form(unsafe { WIPE_SCR_END as _ }, width / 2, height);
    unsafe {
        Y = doom::Z_Malloc(width * mem::size_of::<c_int>() as c_int, doom::PU_STATIC as _, 0 as _) as _;
        *Y = -(random::m_random() % 16);
        for i in 1..width as _ {
            let r = (random::m_random() % 3) - 1;
            *Y.offset(i) = *Y.offset(i - 1) + r;
            if *Y.offset(i) > 0 {
                *Y.offset(i) = 0;
            } else if *Y.offset(i) == -16 {
                *Y.offset(i) = -15;
            }
        }
    }
    0
}

#[export_name = "wipe_doMelt"]
pub extern "C" fn do_melt(width: c_int, height: c_int, ticks: c_int) -> c_int {
    let y_slice = unsafe { slice::from_raw_parts_mut(Y as *mut c_short, width as _) };
    let mut done = true;
    let half_width = width / 2;
    let mut remaining = ticks;
    while remaining != 0 {
        for i in 0..half_width as usize {
            unsafe {
                if y_slice[i] < 0 {
                    y_slice[i] += 1;
                    done = false;
                } else if y_slice[i] < height as c_short {
                    let mut dy = if y_slice[i] < 16 {
                        y_slice[i] + 1
                    } else {
                        8
                    };
                    if y_slice[i] + dy >= height as c_short {
                        dy = height as c_short - y_slice[i];
                    }
                    let mut s = WIPE_SCR_END.offset((i * height as usize + y_slice[i] as usize) as isize * mem::size_of::<c_short>() as isize) as *mut c_short;
                    let mut d = WIPE_SCR.offset((y_slice[i] as usize * half_width as usize + i) as isize * mem::size_of::<c_short>() as isize) as *mut c_short;
                    let mut idx = 0;
                    for _ in (0..dy).rev() {
                        *d.offset(idx) = *s;
                        s = s.offset(1);
                        idx += half_width as isize;
                    }
                    y_slice[i] += dy;
                    s = WIPE_SCR_START.offset(i as isize * height as isize * mem::size_of::<c_short>() as isize) as *mut c_short;
                    d = WIPE_SCR.offset((y_slice[i] as isize * half_width as isize + i as isize) * mem::size_of::<c_short>() as isize) as *mut c_short;
                    idx = 0;
                    for _ in (0..(height - y_slice[i] as c_int)).rev() {
                        *d.offset(idx) = *s;
                        s = s.offset(1);
                        idx += half_width as isize;
                    }
                    done = false;
                }
            }
        }
        remaining -= 1;
    }
    done as _
}

#[export_name = "wipe_exitMelt"]
// TODO: More unused arguments/return values to deal with. ~Alex
pub extern "C" fn exit_melt(_width: c_int, _height: c_int, _ticks: c_int) -> c_int {
    unsafe {
        doom::Z_Free(Y as _);
    }
    0
}

#[export_name = "wipe_StartScreen"]
// TODO: More unused arguments/return values to deal with. ~Alex
pub extern "C" fn start_screen(_x: c_int, _y: c_int, _width: c_int, _height: c_int) -> c_int {
    unsafe {
        WIPE_SCR_START = doom::screens[2];
        doom::I_ReadScreen(WIPE_SCR_START);
    }
    0
}

#[export_name = "wipe_EndScreen"]
pub extern "C" fn end_screen(x: c_int, y: c_int, width: c_int, height: c_int, _ticks: c_int) -> c_int {
    unsafe {
        WIPE_SCR_END = doom::screens[3];
        doom::I_ReadScreen(WIPE_SCR_END);
        doom::V_DrawBlock(x, y, 0, width, height, WIPE_SCR_START);
    }
    0
}

#[export_name = "wipe_ScreenWipe"]
pub extern "C" fn screen_wipe(wipeno: c_int, _x: c_int, _y: c_int, width: c_int, height: c_int, ticks: c_int) -> c_int {
    let wipes = [
        init_color_x_form,
        do_color_x_form,
        exit_color_x_form,
        init_melt,
        do_melt,
        exit_melt
    ];
    if unsafe { !GO } {
        unsafe {
            GO = true;
            WIPE_SCR = doom::screens[0];
        }
        (wipes[wipeno as usize * 3])(width, height, ticks);
    }
    unsafe {
        doom::V_MarkRect(0, 0, width, height);
    }
    let rc = (wipes[wipeno as usize * 3 + 1])(width, height, ticks);
    if rc != 0 {
        unsafe { GO = false };
        (wipes[wipeno as usize * 3 + 2])(width, height, ticks);
    }
    unsafe { !GO as _ }
}