use crate::config::StartupConfig;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

/// Lance tous les exécutables de la liste.
/// Retourne `false` si la liste est vide, `true` sinon (les échecs individuels sont loggés en WARN).
pub fn launch(apps: &[String]) -> bool {
    if apps.is_empty() { return false; }
    for path in apps {
        if path.is_empty() { continue; }
        if !Path::new(path).exists() {
            log::warn!("startup: not found: {}", path);
            continue;
        }
        match Command::new(path)
            .creation_flags(0x08000000)
            .spawn()
        {
            Ok(child) => log::info!("startup: spawned {} pid={}", path, child.id()),
            Err(e)    => log::warn!("startup: spawn error {}: {}", path, e),
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
