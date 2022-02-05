use crate::{info::StateNumber, types::{AmmoType, WeaponInfo}};

#[export_name = "weaponinfo"]
#[used]
pub static WEAPON_INFO: [WeaponInfo; 9] = [
    // Fists
    WeaponInfo {
        ammo: AmmoType::NoAmmo,
        up_state: StateNumber::S_PUNCHUP,
        down_state: StateNumber::S_PUNCHDOWN,
        ready_state: StateNumber::S_PUNCH,
        attack_state: StateNumber::S_PUNCH1,
        flash_state: StateNumber::S_NULL
    },
    // Pistol
    WeaponInfo {
        ammo: AmmoType::Clip,
        up_state: StateNumber::S_PISTOLUP,
        down_state: StateNumber::S_PISTOLDOWN,
        ready_state: StateNumber::S_PISTOL,
        attack_state: StateNumber::S_PISTOL1,
        flash_state: StateNumber::S_PISTOLFLASH
    },
    // Shotgun
    WeaponInfo {
        ammo: AmmoType::Shell,
        up_state: StateNumber::S_SGUNUP,
        down_state: StateNumber::S_SGUNDOWN,
        ready_state: StateNumber::S_SGUN,
        attack_state: StateNumber::S_SGUN1,
        flash_state: StateNumber::S_SGUNFLASH1
    },
    // Chaingun
    WeaponInfo {
        ammo: AmmoType::Clip,
        up_state: StateNumber::S_CHAINUP,
        down_state: StateNumber::S_CHAINDOWN,
        ready_state: StateNumber::S_CHAIN,
        attack_state: StateNumber::S_CHAIN1,
        flash_state: StateNumber::S_CHAINFLASH1
    },
    // Missile Launcher
    WeaponInfo {
        ammo: AmmoType::Missile,
        up_state: StateNumber::S_MISSILEUP,
        down_state: StateNumber::S_MISSILEDOWN,
        ready_state: StateNumber::S_MISSILE,
        attack_state: StateNumber::S_MISSILE1,
        flash_state: StateNumber::S_MISSILEFLASH1
    },
    // Plasma Rifle
    WeaponInfo {
        ammo: AmmoType::Cell,
        up_state: StateNumber::S_PLASMAUP,
        down_state: StateNumber::S_PLASMADOWN,
        ready_state: StateNumber::S_PLASMA,
        attack_state: StateNumber::S_PLASMA1,
        flash_state: StateNumber::S_PLASMAFLASH1
    },
    // BFG 9000
    WeaponInfo {
        ammo: AmmoType::Cell,
        up_state: StateNumber::S_BFGUP,
        down_state: StateNumber::S_BFGDOWN,
        ready_state: StateNumber::S_BFG,
        attack_state: StateNumber::S_BFG1,
        flash_state: StateNumber::S_BFGFLASH1
    },
    // Chainsaw
    WeaponInfo {
        ammo: AmmoType::NoAmmo,
        up_state: StateNumber::S_SAWUP,
        down_state: StateNumber::S_SAWDOWN,
        ready_state: StateNumber::S_SAW,
        attack_state: StateNumber::S_SAW1,
        flash_state: StateNumber::S_NULL
    },
    // Super Shotgun
    WeaponInfo {
        ammo: AmmoType::Shell,
        up_state: StateNumber::S_DSGUNUP,
        down_state: StateNumber::S_DSGUNDOWN,
        ready_state: StateNumber::S_DSGUN,
        attack_state: StateNumber::S_DSGUN1,
        flash_state: StateNumber::S_DSGUNFLASH1
    }
];