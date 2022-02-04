use std::os::raw::c_int;

#[export_name = "skyflatnum"]
#[used]
pub static mut SKY_FLAT_NUM: c_int = 0;

#[export_name = "skytexture"]
#[used]
pub static mut SKY_TEXTURE: c_int = 0;

#[export_name = "skytexturemid"]
#[used]
pub static mut SKY_TEXTURE_MID: c_int = 0;

#[export_name = "R_InitSkyMap"]
pub extern "C" fn init_sky_map() {
    unsafe {
        SKY_TEXTURE_MID = 100 * crate::doom::FRACUNIT as c_int;
    }
}