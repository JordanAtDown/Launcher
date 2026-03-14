use crate::config::UpdatesConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;

/// Arrête le service Windows Update (wuauserv) via `net stop`.
/// Évite les téléchargements et redémarrages intempestifs en session de jeu.
/// Ne fait rien si `pause_in_game = false` dans la config.
pub fn pause(cfg: &UpdatesConfig) -> bool {
    if !cfg.pause_in_game {
        return false;
    }
    match Command::new("net")
        .args(["stop", "wuauserv"])
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s)  => { log::warn!("updates: wuauserv exit={:?}", s.code()); false }
        Err(e) => { log::warn!("updates: error: {}", e); false }
    }
}
