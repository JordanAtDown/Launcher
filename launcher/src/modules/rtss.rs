use crate::config::RtssConfig;
use std::fs;
use std::os::windows::process::CommandExt;
use std::process::Command;

/// Met à jour la limite de framerate dans le profil Global de RTSS (si configuré),
/// puis lance RTSS.exe. Retourne `false` si le chemin est absent ou introuvable.
pub fn apply(cfg: &RtssConfig, limit: Option<u32>) -> bool {
    if let Some(limit) = limit {
        if let Some(profile_path) = &cfg.profile_path {
            if let Ok(content) = fs::read_to_string(profile_path) {
                let new_content: String = content
                    .lines()
                    .map(|line| {
                        if line.trim_start().starts_with("FramerateLimit=") {
                            format!("FramerateLimit={}", limit)
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\r\n");
                let _ = fs::write(profile_path, new_content + "\r\n");
            }
        }
    }

    if let Some(exe_path) = &cfg.path {
        if !exe_path.is_empty() {
            if !std::path::Path::new(exe_path).exists() {
                log::warn!("rtss: path not found: {}", exe_path);
                return false;
            }
            return match Command::new(exe_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .spawn()
            {
                Ok(child) => { log::info!("rtss spawned pid={}", child.id()); true }
                Err(e)    => { log::warn!("rtss spawn error: {}", e); false }
            };
        }
    }

    false
}
