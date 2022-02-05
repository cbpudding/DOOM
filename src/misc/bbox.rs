use crate::types::Fixed;
use std::{os::raw::c_int, slice};

const BOX_TOP: usize = 0;
const BOX_BOTTOM: usize = 1;
const BOX_LEFT: usize = 2;
const BOX_RIGHT: usize = 3;

#[export_name = "M_ClearBox"]
pub extern "C" fn clear_box(bbox: *mut Fixed) {
    let bounding_box = unsafe { slice::from_raw_parts_mut(bbox, 4) };
    bounding_box[BOX_TOP] = c_int::MIN;
    bounding_box[BOX_RIGHT] = c_int::MIN;
    bounding_box[BOX_BOTTOM] = c_int::MAX;
    bounding_box[BOX_LEFT] = c_int::MAX;
}

#[export_name = "M_AddToBox"]
pub extern "C" fn add_to_box(bbox: *mut Fixed, x: Fixed, y: Fixed) {
    let bounding_box = unsafe { slice::from_raw_parts_mut(bbox, 4) };
    if x < bounding_box[BOX_LEFT] {
        bounding_box[BOX_LEFT] = x;
    } else if x > bounding_box[BOX_RIGHT] {
        bounding_box[BOX_RIGHT] = x;
    }
    if y < bounding_box[BOX_BOTTOM] {
        bounding_box[BOX_BOTTOM] = y;
    } else if y > bounding_box[BOX_TOP] {
        bounding_box[BOX_TOP] = y;
    }
}