use crate::info::StateNumber;
use std::os::raw::c_int;

pub type Fixed = c_int;

#[repr(C)]
pub enum AmmoType {
    Clip,
    Shell,
    Cell,
    Missile,
    NoAmmo = 6
}

#[repr(C)]
pub enum GameMode {
    Shareware,
    Registered,
    Commercial,
    Retail,
    Indetermined
}

#[repr(C)]
pub enum GameMission {
    Doom,
    Doom2,
    PackTnt,
    PackPlut,
    None
}

#[repr(C)]
pub enum Language {
    English,
    French,
    German,
    Unknown
}

#[repr(C)]
pub struct WeaponInfo {
    pub ammo: AmmoType,
    pub up_state: StateNumber,
    pub down_state: StateNumber,
    pub ready_state: StateNumber,
    pub attack_state: StateNumber,
    pub flash_state: StateNumber
}