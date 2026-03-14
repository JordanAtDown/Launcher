use crate::config::MonitorConfig;
use std::process::Command;

/// Détermine si le launcher doit s'exécuter en mode jeu (`true`) ou bureau (`false`).
/// En mode `"auto"`, détecte si un moniteur dont le nom contient `name` est connecté.
/// Les modes `"game"` et `"desktop"` forcent le résultat sans détection.
pub fn resolve_mode(cfg: &MonitorConfig) -> bool {
    match cfg.mode.as_deref().unwrap_or("auto") {
        "game"    => true,
        "desktop" => false,
        _         => detect(cfg.name.as_deref().unwrap_or("")),
    }
}

fn detect(name: &str) -> bool {
    if name.is_empty() { return false; }
    let output = Command::new("powershell")
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            "Get-WmiObject Win32_DesktopMonitor | Select-Object -ExpandProperty Name",
        ])
        .output();
    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .to_lowercase()
            .contains(&name.to_lowercase()),
        Err(_) => false,
    }
}
