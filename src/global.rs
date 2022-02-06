use crate::types::{GameMission, GameMode, Language};
use std::{os::raw::{c_char, c_int}, ptr};

pub const TICK_RATE: c_int = 35;

#[export_name = "gamemode"]
#[used]
pub static mut GAMEMODE: GameMode = GameMode::Indetermined;

#[export_name = "gamemission"]
#[used]
pub static mut GAMEMISSION: GameMission = GameMission::None;

#[export_name = "language"]
#[used]
pub static mut LANGUAGE: Language = Language::English;

#[export_name = "modifiedgame"]
#[used]
pub static mut MODIFIED_GAME: bool = false;

#[export_name = "myargc"]
#[used]
pub static mut MY_ARGC: c_int = 0;

#[export_name = "myargv"]
#[used]
pub static mut MY_ARGV: *mut *mut c_char = ptr::null_mut();