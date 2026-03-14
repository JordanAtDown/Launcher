use crate::config::AfterburnerConfig;
use std::path::Path;
use std::process::Command;

/// Lance MSI Afterburner, optionnellement avec un profil d'overclocking (/Profile1..5).
/// Retourne `false` si le chemin est absent ou introuvable sur le disque.
pub fn launch(cfg: &AfterburnerConfig, profile: Option<u8>) -> bool {
    let path = match &cfg.path {
        Some(p) => p,
        None => return false,
    };
    if !Path::new(path).exists() {
        log::warn!("afterburner: path not found: {}", path);
        return false;
    }
    let mut cmd = Command::new(path);
    if let Some(p) = profile {
        cmd.arg(format!("/Profile{}", p));
    }
    match cmd.spawn() {
        Ok(child) => { log::info!("afterburner spawned pid={}", child.id()); true }
        Err(e)    => { log::warn!("afterburner spawn error: {}", e); false }
    }
}
