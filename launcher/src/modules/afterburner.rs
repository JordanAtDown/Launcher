use crate::config::AfterburnerConfig;
use std::path::Path;
use super::spawn_minimized;

/// Lance MSI Afterburner minimisé, optionnellement avec un profil d'overclocking (/Profile1..5).
/// Retourne `false` si le chemin est absent ou introuvable sur le disque.
pub fn launch(cfg: &AfterburnerConfig, profile: Option<u8>) -> bool {
    let path = match &cfg.path {
        Some(p) => p.as_str(),
        None => return false,
    };
    if !Path::new(path).exists() {
        log::warn!("afterburner: path not found: {}", path);
        return false;
    }
    let profile_arg;
    let args: &[&str] = if let Some(p) = profile {
        profile_arg = format!("/Profile{}", p);
        &[profile_arg.as_str()]
    } else {
        &[]
    };
    match spawn_minimized(path, args) {
        Some(pid) => { log::info!("afterburner spawned pid={}", pid); true }
        None      => { log::warn!("afterburner spawn error"); false }
    }
}
