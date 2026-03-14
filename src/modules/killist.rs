use crate::config::KilllistConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;

/// Arrête les services Windows (`sc stop`) et tue les processus (`taskkill /F`)
/// listés dans la config. Libère RAM et CPU avant de lancer le jeu.
/// Retourne toujours `true` — les échecs individuels sont loggés en WARN.
pub fn apply(cfg: &KilllistConfig) -> bool {
    for service in &cfg.services {
        match Command::new("sc")
            .args(["stop", service])
            .creation_flags(0x08000000)
            .status()
        {
            Ok(s) if s.success() => {}
            Ok(s)  => log::warn!("killist: sc stop {} exit={:?}", service, s.code()),
            Err(e) => log::warn!("killist: sc stop {} error: {}", service, e),
        }
    }
    for process in &cfg.processes {
        match Command::new("taskkill")
            .args(["/F", "/IM", process])
            .creation_flags(0x08000000)
            .status()
        {
            Ok(s) if s.success() => {}
            Ok(s)  => log::warn!("killist: taskkill {} exit={:?}", process, s.code()),
            Err(e) => log::warn!("killist: taskkill {} error: {}", process, e),
        }
    }
    true
}
