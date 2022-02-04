use crate::types::{GameMission, GameMode, Language};

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
