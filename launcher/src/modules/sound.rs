use crate::config::SoundConfig;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

/// Cherche le nom exact d'un périphérique audio dont le nom contient `needle` (case-insensitive).
/// Utilise `svcl.exe /scomma ""` pour lister tous les périphériques actifs en CSV.
/// Retourne `None` si le périphérique est absent ou si l'exe plante.
fn find_device(exe_path: &str, needle: &str) -> Option<String> {
    let output = Command::new(exe_path)
        .args(["/scomma", ""])
        .creation_flags(0x08000000)
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let name = line.split(',').next()?.trim();
        if name.to_lowercase().contains(&needle.to_lowercase()) {
            return Some(name.to_string());
        }
    }
    None
}

fn run_set_default(exe_path: &str, device_name: &str) -> bool {
    match Command::new(exe_path)
        .args(["/SetDefault", device_name, "all"])
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s) => { log::warn!("sound: /SetDefault exit={:?}", s.code()); false }
        Err(e) => { log::warn!("sound: error: {}", e); false }
    }
}

fn set_device(exe_path: &str, device: &str, auto_detect: bool) -> bool {
    if auto_detect {
        match find_device(exe_path, device) {
            Some(exact) => {
                log::info!("sound: device found: \"{}\"", exact);
                run_set_default(exe_path, &exact)
            }
            None => {
                log::warn!("sound: device not found: \"{}\"", device);
                false
            }
        }
    } else {
        run_set_default(exe_path, device)
    }
}

/// Redirige la sortie audio principale vers `game_device` (ex: TV LG via HDMI).
/// En mode `auto_detect`, vérifie d'abord que le périphérique est visible par Windows :
/// si la TV est éteinte ou débranchée, retourne `false` sans tenter le switch.
pub fn set_game(cfg: &SoundConfig) -> bool {
    let device = match &cfg.game_device { Some(d) if !d.is_empty() => d, _ => return false };
    let exe    = match &cfg.exe_path    { Some(p) if !p.is_empty() => p, _ => return false };
    if !Path::new(exe).exists() {
        log::warn!("sound: exe not found: {}", exe);
        return false;
    }
    set_device(exe, device, cfg.auto_detect)
}

/// Redirige la sortie audio principale vers `desktop_device` (ex: enceintes).
/// Retourne `false` si l'exe est absent ou si `desktop_device` n'est pas configuré.
pub fn set_desktop(cfg: &SoundConfig) -> bool {
    let device = match &cfg.desktop_device { Some(d) if !d.is_empty() => d, _ => return false };
    let exe    = match &cfg.exe_path       { Some(p) if !p.is_empty() => p, _ => return false };
    if !Path::new(exe).exists() {
        log::warn!("sound: exe not found: {}", exe);
        return false;
    }
    set_device(exe, device, cfg.auto_detect)
}