// cec-daemon — daemon Windows pour le contrôle TV HDMI CEC
//
// Usage : cec-daemon.exe --path "<chemin vers cec-client.exe>"
//
// Événements gérés :
//   GUID_CONSOLE_DISPLAY_STATE OFF → standby TV  (couvre S3, Modern Standby S0, inactivité)
//   GUID_CONSOLE_DISPLAY_STATE ON  → allume TV + source HDMI active
//   CTRL_SHUTDOWN_EVENT / CTRL_LOGOFF_EVENT → standby TV avant extinction

#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

mod cec;
mod logging;
mod power;

use cec::{send_cec, CecClient};
use windows_sys::Win32::System::Console::{
    SetConsoleCtrlHandler, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
};

// ─── Gestionnaire d'arrêt système ─────────────────────────────────────────────

/// Callback appelé par Windows lors d'un arrêt (`CTRL_SHUTDOWN_EVENT`) ou
/// d'une déconnexion de session (`CTRL_LOGOFF_EVENT`).
///
/// Envoie la commande standby à la TV et laisse 500 ms à CEC pour la transmettre
/// avant que Windows termine le processus.
///
/// # Safety
/// Callback Win32 — signature imposée par `SetConsoleCtrlHandler`.
unsafe extern "system" fn ctrl_handler(ctrl_type: u32) -> i32 {
    match ctrl_type {
        CTRL_SHUTDOWN_EVENT | CTRL_LOGOFF_EVENT => {
            log::info!("cec-daemon: shutdown/logoff → standby");
            send_cec("standby 0");
            // Délai pour laisser le message CEC transiter avant la fin forcée du processus.
            std::thread::sleep(std::time::Duration::from_millis(500));
            0 // 0 = laisser les autres handlers traiter l'événement
        }
        _ => 0,
    }
}

// ─── Point d'entrée ───────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::setup_logging();

    // ── Parse --path <cec-client.exe> ──────────────────────────────────────────
    let args: Vec<String> = std::env::args().collect();
    let cec_path = args
        .windows(2)
        .find(|w| w[0] == "--path")
        .map(|w| w[1].clone())
        .ok_or("usage: cec-daemon.exe --path <cec-client.exe>")?;

    // ── Spawn cec-client et initialise le global stdin ─────────────────────────
    let client = CecClient::spawn(&cec_path)?;
    cec::init_global_stdin(client.stdin.clone());
    log::info!("cec-daemon: started, cec-client pid={}", client.child.id());

    // ── Enregistrement du handler d'arrêt système ──────────────────────────────
    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_handler), 1);
    }

    // ── Création de la fenêtre cachée + notification d'état de l'écran ─────────
    let _hwnd = unsafe { power::create_power_window()? };
    log::info!("cec-daemon: listening for display/shutdown events");

    // ── Boucle de messages (bloquant jusqu'à WM_QUIT) ──────────────────────────
    unsafe { power::run_message_loop() };

    // ── Nettoyage : standby TV + arrêt propre de cec-client ───────────────────
    log::info!("cec-daemon: message loop ended, sending standby and exiting");
    send_cec("standby 0");
    std::thread::sleep(std::time::Duration::from_millis(300));
    client.shutdown();

    Ok(())
}
