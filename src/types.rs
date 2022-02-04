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