use crate::config::GameBarConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

/// Désinstalle le package Xbox Gaming Overlay (overlay + widget Xbox) via PowerShell AppX,
/// et désactive Game DVR (capture vidéo) via le registre HKCU. Opération idempotente.
/// Ne fait rien si `uninstall = false` dans la config.
pub fn uninstall(cfg: &GameBarConfig) -> bool {
    if !cfg.uninstall {
        return false;
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Désinstaller le package Xbox Gaming Overlay (idempotent)
    let ok = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-AppxPackage Microsoft.XboxGamingOverlay | Remove-AppxPackage",
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .is_ok();

    // Désactiver Game DVR via le registre
    if let Ok(key) = hkcu.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        KEY_SET_VALUE,
    ) {
        let _ = key.set_value("AppCaptureEnabled", &0u32);
    }
    if let Ok(key) = hkcu.open_subkey_with_flags(r"System\GameConfigStore", KEY_SET_VALUE) {
        let _ = key.set_value("GameDVR_Enabled", &0u32);
    }

    ok
}
