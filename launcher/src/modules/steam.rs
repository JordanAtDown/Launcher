use crate::config::SteamConfig;
use std::path::Path;
use std::process::Command;

/// Lance Steam avec les arguments configurés.
/// Retourne `false` si le chemin est vide ou introuvable sur le disque.
pub fn launch(cfg: &SteamConfig) -> bool {
    if cfg.path.is_empty() {
        return false;
    }
    if !Path::new(&cfg.path).exists() {
        log::warn!("steam: path not found: {}", cfg.path);
        return false;
    }
    match Command::new(&cfg.path).args(&cfg.args).spawn() {
        Ok(child) => { log::info!("steam spawned pid={}", child.id()); true }
        Err(e)    => { log::warn!("steam spawn error: {}", e); false }
    }
}
