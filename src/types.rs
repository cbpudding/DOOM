use crate::info::StateNumber;
use std::os::raw::{c_char, c_int, c_short};

pub type Fixed = c_int;

#[repr(C)]
pub enum AmmoType {
    Clip,
    Shell,
    Cell,
    Missile,
    NoAmmo = 5,
}

#[repr(C)]
pub struct CheatSeq {
    pub sequence: *mut c_char,
    pub p: *mut c_char,
}

#[repr(C)]
pub enum GameMode {
    Shareware,
    Registered,
    Commercial,
    Retail,
    Indetermined,
}

#[repr(C)]
pub enum GameMission {
    Doom,
    Doom2,
    PackTnt,
    PackPlut,
    None,
}

#[repr(C)]
pub enum Language {
    English,
    French,
    German,
    Unknown,
}

#[repr(C)]
pub struct Ticcmd {
    pub forward_move: c_char,
    pub side_move: c_char,
    pub angle_turn: c_short,
    pub consistency: c_short,
    pub chat_char: u8,
    pub buttons: u8,
}

#[repr(C)]
pub struct WeaponInfo {
    pub ammo: AmmoType,
    pub up_state: StateNumber,
    pub down_state: StateNumber,
    pub ready_state: StateNumber,
    pub attack_state: StateNumber,
    pub flash_state: StateNumber,
}
