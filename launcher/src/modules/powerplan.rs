use std::os::windows::process::CommandExt;
use std::process::Command;

/// Active un plan d'alimentation Windows via `powercfg /setactive <GUID>`.
/// Lister les GUIDs disponibles : `powercfg /L` dans un terminal admin.
/// Retourne `false` si le GUID est vide (module désactivé).
pub fn apply(guid: &str) -> bool {
    if guid.is_empty() {
        return false;
    }
    match Command::new("powercfg")
        .args(["/setactive", guid])
        .creation_flags(0x08000000)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s)  => { log::warn!("powerplan: powercfg exit={:?}", s.code()); false }
        Err(e) => { log::warn!("powerplan: error: {}", e); false }
    }
}
