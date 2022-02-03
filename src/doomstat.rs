use crate::types::{GameMode, GameMission, Language};

#[link(name = "gamemode")]
#[used]
pub static mut GAMEMODE: GameMode = GameMode::Indetermined;

#[link(name = "gamemission")]
#[used]
pub static mut GAMEMISSION: GameMission = GameMission::None;

#[link(name = "language")]
#[used]
pub static mut LANGUAGE: Language = Language::English;

#[link(name = "modifiedgame")]
#[used]
pub static mut MODIFIED_GAME: bool = false;