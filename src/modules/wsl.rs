use crate::config::WslConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;

/// Arrête tous les distros WSL et la VM légère via `wsl --shutdown`.
/// Retourne `false` si désactivé dans la config ou si la commande échoue.
pub fn shutdown(cfg: &WslConfig) -> bool {
    if !cfg.enabled { return false; }
    match Command::new("wsl")
        .args(["--shutdown"])
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => { log::info!("wsl: shutdown ok"); true }
        Ok(s)  => { log::warn!("wsl: shutdown exit={:?}", s.code()); false }
        Err(e) => { log::warn!("wsl: shutdown error: {}", e); false }
    }
}
