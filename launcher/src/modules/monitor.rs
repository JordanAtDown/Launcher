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
    // WmiMonitorID lit les noms EDID réels (fabricant + modèle) contrairement à
    // Win32_DesktopMonitor qui retourne uniquement "Moniteur Plug-and-Play générique"
    let cmd = r#"Get-WmiObject -Namespace root\wmi -Class WmiMonitorID | ForEach-Object { ($_.UserFriendlyName | Where-Object {$_} | ForEach-Object {[char]$_}) -join '' }"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", cmd])
        .output();
    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .to_lowercase()
            .contains(&name.to_lowercase()),
        Err(_) => false,
    }
}
