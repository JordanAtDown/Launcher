use crate::config::NotificationsConfig;
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

/// Désactive les notifications toast Windows via le registre HKCU (ToastEnabled=0).
/// Ne fait rien si `disable_in_game = false` dans la config.
pub fn disable(cfg: &NotificationsConfig) -> bool {
    if !cfg.disable_in_game { return false; }
    set_toast_enabled(0)
}

/// Réactive les notifications toast Windows (ToastEnabled=1).
/// Appelé en mode bureau pour restaurer le comportement normal.
/// Ne fait rien si `disable_in_game = false` dans la config.
pub fn restore(cfg: &NotificationsConfig) -> bool {
    if !cfg.disable_in_game { return false; }
    set_toast_enabled(1)
}

fn set_toast_enabled(value: u32) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\PushNotifications",
        KEY_SET_VALUE,
    ) {
        return key.set_value("ToastEnabled", &value).is_ok();
    }
    false
}
