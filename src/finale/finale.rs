use std::{os::raw::*, ptr::{null, null_mut}};

use crate::doom::{self, OF_StartFinale, event_t, OF_Responder, OF_Ticker, OF_TextWrite, OF_StartCast, OF_CastTicker, OF_CastResponder, OF_CastPrint, OF_CastDrawer, OF_DrawPatchCol, OF_BunnyScroll, OF_Drawer, E1TEXT, musicenum_t_mus_victor, musicenum_t, E2TEXT, E3TEXT, E4TEXT, musicenum_t_mus_read_m, C1TEXT, C2TEXT, C3TEXT, C4TEXT, C5TEXT, C6TEXT, gameaction, gameaction_t_ga_nothing, gamestate, gamestate_t_GS_FINALE, viewactive, automapactive, gamemode, GameMode_t_shareware, GameMode_t_registered, GameMode_t_retail, GameMode_t_commercial, gameepisode, gamemap, S_ChangeMusic, F_CastResponder};
use crate::doom::state_t;
use crate::doom::patch_t;

#[export_name = "finalestage"]
pub static mut FINALE_STAGE: c_int = 0;
#[export_name = "finalecount"]
pub static mut FINALE_COUNT: c_int = 0;

#[export_name = "finaletext"]
pub static mut FINALE_TEXT: *const char = null();
#[export_name = "finaleflat"]
pub static mut FINALE_FLAT: *const char = null();

#[export_name = "castnum"]
pub static mut CASTNUM: c_int = 0;
#[export_name = "casttics"]
pub static mut CASTTICS: c_int = 0;
#[export_name = "caststate"]
pub static mut CASTSTATE: *mut state_t = null_mut();
#[export_name = "castdeath"]
pub static mut CASTDEATH: doom::boolean = 0;
#[export_name = "castframes"]
pub static mut CASTFRAMES: c_int = 0;
#[export_name = "castonmelee"]
pub static mut CASTONMELEE: c_int = 0;
#[export_name = "castattacking"]
pub static mut CASTATTACKING: doom::boolean = 0;

pub const DOOM1_MUS: musicenum_t = musicenum_t_mus_victor;
pub const DOOM1: [(&[u8], &[u8]); 4] = [
    (b"FLOOR4_8", E1TEXT),
    (b"SFLR6_1", E2TEXT),
    (b"MFLR8_4", E3TEXT),
    (b"MFLR8_3", E4TEXT),
];

pub const DOOM2_MUS: musicenum_t = musicenum_t_mus_read_m;
pub const DOOM2: [(&[u8], &[u8]); 6] = [
    (b"SLIME16", C1TEXT),
    (b"RROCK14", C2TEXT),
    (b"RROCK07", C3TEXT),
    (b"RROCK17", C4TEXT),
    (b"RROCK13", C5TEXT),
    (b"RROCK19", C6TEXT),
];
pub const DOOM2_LEV: [c_int; 6] = [
    6,
    11,
    20,
    30,
    15,
    31,
];

pub const UKWN_MUS: musicenum_t = musicenum_t_mus_read_m;
pub const UKWN: (&[u8], &[u8]) = (b"F_SKY1", C1TEXT);

#[export_name = "F_StartFinale"]
pub unsafe extern "C" fn start_finale() {
    gameaction = gameaction_t_ga_nothing;
    gamestate = gamestate_t_GS_FINALE;
    viewactive = 0;
    automapactive = 0;

    let (mus, (flat, text)) = match gamemode {
        GameMode_t_shareware | GameMode_t_registered | GameMode_t_retail => {
            (DOOM1_MUS, DOOM1[gameepisode as usize - 1])
        },
        GameMode_t_commercial => {
            let mut data = UKWN;
            for (i, lev) in DOOM2_LEV.iter().enumerate() {
                if *lev == gamemap {
                    data = DOOM2[i];
                    break;
                }
            }

            (DOOM2_MUS, data)
        },
        _ => (UKWN_MUS, UKWN)
    };

    S_ChangeMusic(mus as _, 1);
    FINALE_FLAT = flat.as_ptr() as _;
    FINALE_TEXT = text.as_ptr() as _;

    FINALE_STAGE = 0;
    FINALE_COUNT = 0;
}

#[export_name = "F_Responder"]
pub unsafe extern "C" fn responder(event: *mut event_t) -> doom::boolean {
    if FINALE_STAGE == 2 {
        F_CastResponder(event)
    } else {
        0
    }
}

#[export_name = "F_Ticker"]
pub unsafe extern "C" fn ticker() {
    OF_Ticker()
}

#[export_name = "F_TextWrite"]
pub unsafe extern "C" fn text_write() {
    OF_TextWrite()
}

#[export_name = "F_StartCast"]
pub unsafe extern "C" fn start_cast() {
    OF_StartCast()
}

#[export_name = "F_CastTicker"]
pub unsafe extern "C" fn cast_ticker() {
    OF_CastTicker()
}

#[export_name = "F_CastResponder"]
pub unsafe extern "C" fn cast_responder(ev: *mut event_t) -> doom::boolean {
    OF_CastResponder(ev)
}

#[export_name = "F_CastPrint"]
pub unsafe extern "C" fn cast_print(text: *mut i8) {
    OF_CastPrint(text)
}

#[export_name = "F_CastDrawer"]
pub unsafe extern "C" fn cast_drawer() {
    OF_CastDrawer()
}

#[export_name = "F_DrawPatchCol"]
pub unsafe extern "C" fn draw_patch_col(x: c_int, patch: *mut patch_t, col: c_int) {
    OF_DrawPatchCol(x, patch, col)
}

#[export_name = "F_BunnyScroll"]
pub unsafe extern "C" fn bunny_scroll() {
    OF_BunnyScroll()
}

#[export_name = "F_Drawer"]
pub unsafe extern "C" fn drawer() {
    OF_Drawer()
}
