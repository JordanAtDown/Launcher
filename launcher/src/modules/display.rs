use crate::config::DisplayConfig;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn apply_resolution(exe_path: &Option<String>, width: Option<u32>, height: Option<u32>, refresh: u32) -> bool {
    let (exe, w, h) = match (exe_path.as_deref(), width, height) {
        (Some(e), Some(w), Some(h)) if !e.is_empty() => (e, w, h),
        _ => return false,
    };
    if !Path::new(exe).exists() {
        log::warn!("display: nircmd not found: {}", exe);
        return false;
    }
    let mut args = vec![
        "setdisplay".to_string(),
        w.to_string(),
        h.to_string(),
        "32".to_string(),
    ];
    if refresh > 0 { args.push(refresh.to_string()); }
    match Command::new(exe)
        .args(&args)
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => { log::info!("display: résolution {}x{} appliquée", w, h); true }
        Ok(s) => { log::warn!("display: nircmd exit={:?}", s.code()); false }
        Err(e) => { log::warn!("display: nircmd error: {}", e); false }
    }
}

/// Applique la résolution `game_width x game_height` via nircmd.
pub fn set_game(cfg: &DisplayConfig) -> bool {
    apply_resolution(&cfg.exe_path, cfg.game_width, cfg.game_height, cfg.refresh_rate)
}

/// Applique la résolution `desktop_width x desktop_height` via nircmd.
pub fn set_desktop(cfg: &DisplayConfig) -> bool {
    apply_resolution(&cfg.exe_path, cfg.desktop_width, cfg.desktop_height, cfg.refresh_rate)
}