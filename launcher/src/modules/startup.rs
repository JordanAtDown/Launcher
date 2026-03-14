use crate::config::StartupConfig;
use std::path::Path;
use super::spawn_minimized;

/// Lance tous les exécutables de la liste minimisés.
/// Retourne `false` si la liste est vide, `true` sinon (les échecs individuels sont loggés en WARN).
fn launch(apps: &[String]) -> bool {
    if apps.is_empty() { return false; }
    for path in apps {
        if path.is_empty() { continue; }
        if !Path::new(path).exists() {
            log::warn!("startup: not found: {}", path);
            continue;
        }
        match spawn_minimized(path, &[]) {
            Some(pid) => log::info!("startup: spawned {} pid={}", path, pid),
            None      => log::warn!("startup: spawn error {}", path),
        }
    }
    true
}

pub fn launch_desktop(cfg: &StartupConfig) -> bool {
    launch(&cfg.desktop)
}

pub fn launch_game(cfg: &StartupConfig) -> bool {
    launch(&cfg.game)
}
