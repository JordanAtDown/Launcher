use crate::config::GameModeConfig;
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

/// Active le Game Mode Windows (AutoGameModeEnabled) via le registre HKCU.
/// Priorise le processus de jeu actif et réduit les interruptions OS.
/// Ne fait rien si `enabled = false` dans la config.
pub fn enable(cfg: &GameModeConfig) -> bool {
    if !cfg.enabled {
        return false;
    }
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey_with_flags(r"SOFTWARE\Microsoft\GameBar", KEY_SET_VALUE) {
        return key.set_value("AutoGameModeEnabled", &1u32).is_ok();
    }
    false
}
