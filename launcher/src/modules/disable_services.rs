use crate::config::DisableServicesConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn sc(args: &[&str]) -> bool {
    match Command::new("sc")
        .args(args)
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s)  => { log::warn!("disable_services: sc {} exit={:?}", args.join(" "), s.code()); false }
        Err(e) => { log::warn!("disable_services: sc {} error: {}", args.join(" "), e); false }
    }
}

/// Mode jeu : désactive les services (start= disabled) et les arrête.
/// Retourne toujours `true` — les échecs individuels sont loggés en WARN.
pub fn disable(cfg: &DisableServicesConfig) -> bool {
    if cfg.services.is_empty() { return false; }
    for svc in &cfg.services {
        sc(&["config", svc, "start=", "disabled"]);
        sc(&["stop", svc]);
    }
    true
}

/// Mode bureau : restaure les services en démarrage Manuel (demand).
/// Retourne toujours `true` — les échecs individuels sont loggés en WARN.
pub fn restore(cfg: &DisableServicesConfig) -> bool {
    if cfg.services.is_empty() { return false; }
    for svc in &cfg.services {
        sc(&["config", svc, "start=", "demand"]);
    }
    true
}
