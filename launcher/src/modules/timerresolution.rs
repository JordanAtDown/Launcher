use crate::config::TimerResolutionConfig;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

/// Lance TimerResolution.exe avec la résolution configurée (en ms).
/// Le processus reste actif et restaure automatiquement 15.625ms à sa fermeture.
/// Retourne `false` si le chemin est absent ou introuvable sur le disque.
pub fn apply(cfg: &TimerResolutionConfig) -> bool {
    let path = match &cfg.path {
        Some(p) if !p.is_empty() => p,
        _ => return false,
    };
    if !Path::new(path).exists() {
        log::warn!("timerresolution: path not found: {}", path);
        return false;
    }
    match Command::new(path)
        .args(["--resolution", &cfg.resolution.to_string()])
        .creation_flags(0x08000000)
        .spawn()
    {
        Ok(child) => { log::info!("timerresolution spawned pid={}", child.id()); true }
        Err(e)    => { log::warn!("timerresolution spawn error: {}", e); false }
    }
}