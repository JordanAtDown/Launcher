use crate::config::DisplayConfig;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

/// Convertit un pourcentage de mise à l'échelle en valeur LogPixels Windows.
/// 100%→96 | 125%→120 | 150%→144 | 175%→168 | 200%→192
fn scale_to_logpixels(scale: u32) -> Option<u32> {
    match scale {
        100 => Some(96),
        125 => Some(120),
        150 => Some(144),
        175 => Some(168),
        200 => Some(192),
        _ => {
            log::warn!("display: mise à l'échelle {}% non supportée (valeurs valides : 100, 125, 150, 175, 200)", scale);
            None
        }
    }
}

fn apply_resolution(exe_path: &Option<String>, width: Option<u32>, height: Option<u32>, refresh: u32) -> bool {
    let (exe, w, h) = match (exe_path.as_deref(), width, height) {
        (Some(e), Some(w), Some(h)) if !e.is_empty() => (e, w, h),
        _ => return false,
    };
    if !Path::new(exe).exists() {
        log::warn!("display: QRes not found: {}", exe);
        return false;
    }
    let mut args = vec![
        format!("/x:{}", w),
        format!("/y:{}", h),
        "/c:32".to_string(),
    ];
    if refresh > 0 { args.push(format!("/r:{}", refresh)); }
    match Command::new(exe)
        .args(&args)
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => { log::info!("display: résolution {}x{} appliquée", w, h); true }
        Ok(s) => { log::warn!("display: QRes exit={:?}", s.code()); false }
        Err(e) => { log::warn!("display: QRes error: {}", e); false }
    }
}

fn apply_scale(scale: Option<u32>) -> bool {
    let scale = match scale { Some(s) => s, None => return false };
    let logpixels = match scale_to_logpixels(scale) { Some(v) => v, None => return false };
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey_with_flags(r"Control Panel\Desktop", KEY_SET_VALUE) {
        match key.set_value("LogPixels", &logpixels) {
            Ok(_) => { log::info!("display: mise à l'échelle {}% écrite (LogPixels={}) — effet au prochain démarrage", scale, logpixels); true }
            Err(e) => { log::warn!("display: LogPixels write error: {}", e); false }
        }
    } else {
        log::warn!("display: impossible d'ouvrir Control Panel\\Desktop");
        false
    }
}

/// Applique la résolution `game_width x game_height` (immédiat via QRes)
/// et écrit la mise à l'échelle `game_scale` dans le registre (effet au prochain démarrage).
/// Retourne `false` si aucun des deux n'est configuré ou si les deux échouent.
pub fn set_game(cfg: &DisplayConfig) -> bool {
    let res_ok   = apply_resolution(&cfg.exe_path, cfg.game_width, cfg.game_height, cfg.refresh_rate);
    let scale_ok = apply_scale(cfg.game_scale);
    res_ok || scale_ok
}

/// Applique la résolution `desktop_width x desktop_height` (immédiat via QRes)
/// et écrit la mise à l'échelle `desktop_scale` dans le registre (effet au prochain démarrage).
/// Retourne `false` si aucun des deux n'est configuré ou si les deux échouent.
pub fn set_desktop(cfg: &DisplayConfig) -> bool {
    let res_ok   = apply_resolution(&cfg.exe_path, cfg.desktop_width, cfg.desktop_height, cfg.refresh_rate);
    let scale_ok = apply_scale(cfg.desktop_scale);
    res_ok || scale_ok
}