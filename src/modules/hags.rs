use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_SET_VALUE};
use winreg::RegKey;

fn set_hags(value: u32) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey_with_flags(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        KEY_SET_VALUE,
    ) {
        return key.set_value("HwSchMode", &value).is_ok();
    }
    false
}

/// Active le Hardware-Accelerated GPU Scheduling (HAGS) — délègue la gestion
/// de la mémoire vidéo au GPU, réduit la latence CPU. Nécessite les droits admin.
pub fn enable() -> bool {
    set_hags(2)
}

/// Désactive le Hardware-Accelerated GPU Scheduling — restaure le planificateur
/// traditionnel (valeur par défaut Windows). Nécessite les droits admin.
pub fn disable() -> bool {
    set_hags(1)
}
