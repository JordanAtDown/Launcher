use crate::config::CecConfig;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn run_cmd(cfg: &CecConfig, cmd: &str) -> bool {
    let path = match cfg.client_path.as_deref() {
        Some(p) => p,
        None => {
            log::warn!("cec: client_path non configuré");
            return false;
        }
    };
    if !std::path::Path::new(path).exists() {
        log::warn!("cec: cec-client introuvable: {}", path);
        return false;
    }
    match Command::new(path)
        .args(["-s", "-c", cmd])
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s)  => { log::warn!("cec `{}`: exit={:?}", cmd, s.code()); false }
        Err(e) => { log::warn!("cec `{}`: error: {}", cmd, e); false }
    }
}

/// Allume la TV (CEC opcode IMAGE_VIEW_ON → adresse logique 0).
pub fn power_on(cfg: &CecConfig) -> bool {
    if !cfg.enabled { return true; }
    run_cmd(cfg, "on 0")
}

/// Diffuse Active Source : la TV bascule automatiquement sur l'entrée HDMI du PC.
pub fn set_source(cfg: &CecConfig) -> bool {
    if !cfg.enabled { return true; }
    run_cmd(cfg, "as")
}
