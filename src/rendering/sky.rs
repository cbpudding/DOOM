use std::os::raw::c_int;

#[link(name = "skyflatnum")]
#[used]
pub static mut SKY_FLAT_NUM: c_int = 0;

#[link(name = "skytexture")]
#[used]
pub static mut SKY_TEXTURE: c_int = 0;

#[link(name = "skytexturemid")]
#[used]
pub static mut SKY_TEXTURE_MID: c_int = 0;

#[link(name = "R_InitSkyMap")]
pub extern "C" fn init_sky_map() {
    unsafe {
        SKY_TEXTURE_MID = 100 * crate::doom::FRACUNIT as c_int;
    }
}