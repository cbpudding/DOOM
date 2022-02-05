#[export_name = "SwapSHORT"]
pub extern "C" fn swap_short(victim: u16) -> u16 {
    (victim >> 8) | (victim << 8)
}

#[export_name = "SwapLONG"]
pub extern "C" fn swap_long(victim: u32) -> u32 {
    (victim >> 24) | ((victim >> 8) & 0xFF00) | ((victim << 8) & 0xFF0000) | (victim << 24)
}