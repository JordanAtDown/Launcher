//! Gestion des événements Windows liés à l'alimentation et à l'état de l'écran.
//!
//! Crée une fenêtre cachée pour recevoir les messages `WM_POWERBROADCAST`, puis enregistre
//! `GUID_CONSOLE_DISPLAY_STATE` via `RegisterPowerSettingNotification`. Ce GUID couvre
//! tous les types de veille (S3, Modern Standby S0) ainsi que l'extinction d'écran manuelle.
#![allow(non_snake_case)]

use crate::cec::send_cec;
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::Power::RegisterPowerSettingNotification;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
    RegisterClassExW, CW_USEDEFAULT, DEVICE_NOTIFY_WINDOW_HANDLE, MSG, PBT_POWERSETTINGCHANGE,
    WM_DESTROY, WM_POWERBROADCAST, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// ─── GUID_CONSOLE_DISPLAY_STATE ────────────────────────────────────────────────
//
// {6FE69556-704A-47A0-8F24-C28D936FDA47}
//
// Envoyé via WM_POWERBROADCAST / PBT_POWERSETTINGCHANGE quand l'état de l'écran change.
// Structure du lParam (POWERBROADCAST_SETTING) :
//   [GUID  : 16 bytes] identifiant du paramètre de puissance
//   [Size  :  4 bytes] longueur des données qui suivent
//   [Data  :  4 bytes] u32 : 0 = OFF, 1 = ON, 2 = Dimmed
//
// Ce GUID est préféré à PBT_APMSUSPEND car Modern Standby (S0) — mode veille par défaut
// sur Windows 10/11 — ne déclenche pas PBT_APMSUSPEND.

/// GUID de la notification d'état de l'écran.
pub const DISPLAY_STATE_GUID: GUID = GUID {
    data1: 0x6FE6_9556,
    data2: 0x704A,
    data3: 0x47A0,
    data4: [0x8F, 0x24, 0xC2, 0x8D, 0x93, 0x6F, 0xDA, 0x47],
};

const DISPLAY_OFF: u32 = 0;
const DISPLAY_ON: u32 = 1;

// ─── Helpers internes ──────────────────────────────────────────────────────────

/// Encode une chaîne UTF-8 en UTF-16 null-terminé pour les API Win32.
fn wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Compare deux GUIDs octet par octet.
///
/// # Safety
/// Les deux pointeurs doivent pointer vers des `GUID` valides et alignés.
unsafe fn guid_eq(a: &GUID, b: &GUID) -> bool {
    std::slice::from_raw_parts(a as *const GUID as *const u8, 16)
        == std::slice::from_raw_parts(b as *const GUID as *const u8, 16)
}

// ─── Procédure de fenêtre ──────────────────────────────────────────────────────

/// Procédure de fenêtre cachée qui reçoit les messages de puissance Windows.
///
/// Réagit uniquement à `WM_POWERBROADCAST` / `PBT_POWERSETTINGCHANGE` pour
/// `DISPLAY_STATE_GUID` :
/// - Valeur 0 (OFF) → envoie `standby 0` à la TV
/// - Valeur 1 (ON)  → envoie `on 0` puis `as` (attente 200 ms entre les deux)
///
/// # Safety
/// Callback Win32 — appelé par le dispatcher de messages Windows.
pub unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_POWERBROADCAST && wparam as u32 == PBT_POWERSETTINGCHANGE && lparam != 0 {
        // lParam pointe vers une structure POWERBROADCAST_SETTING :
        //   offset  0 : GUID   (16 bytes) — identifiant du paramètre
        //   offset 16 : DWORD  ( 4 bytes) — longueur des données
        //   offset 20 : DATA   (DataLength bytes)
        let ptr = lparam as *const u8;
        let setting_guid = &*(ptr as *const GUID);

        if guid_eq(setting_guid, &DISPLAY_STATE_GUID) {
            let data_len = *(ptr.add(16) as *const u32);
            if data_len >= 4 {
                let value = *(ptr.add(20) as *const u32);
                match value {
                    DISPLAY_OFF => {
                        log::info!("cec-daemon: display OFF → standby");
                        send_cec("standby 0");
                    }
                    DISPLAY_ON => {
                        log::info!("cec-daemon: display ON → on + active source");
                        send_cec("on 0");
                        // Délai pour laisser la TV s'allumer avant de lui envoyer
                        // la commande Active Source — certains modèles l'ignorent
                        // si elle arrive trop tôt.
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        send_cec("as");
                    }
                    _ => {} // Valeur 2 = Dimmed, ignorée
                }
            }
        }
    } else if msg == WM_DESTROY {
        PostQuitMessage(0);
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}

// ─── Création de la fenêtre de puissance ──────────────────────────────────────

/// Crée une fenêtre cachée et enregistre la notification `GUID_CONSOLE_DISPLAY_STATE`.
///
/// La fenêtre est nécessaire pour recevoir les messages `WM_POWERBROADCAST`.
/// Elle n'est jamais affichée (dimensions 0×0, pas de style visible).
///
/// Retourne le handle `HWND` en cas de succès, ou `Err` si la création échoue.
///
/// # Safety
/// Appelle des fonctions Win32. Doit être appelée depuis le thread qui exécutera
/// la boucle de messages (cf. [`run_message_loop`]).
pub unsafe fn create_power_window() -> Result<HWND, String> {
    let class_name = wstr("CecDaemonWnd");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wnd_proc),
        lpszClassName: class_name.as_ptr(),
        hInstance: 0,
        style: 0,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: 0,
        lpszMenuName: std::ptr::null(),
        hIconSm: 0,
    };
    RegisterClassExW(&wc);

    let hwnd = CreateWindowExW(
        0,
        class_name.as_ptr(),
        wstr("cec-daemon").as_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        0,
        0,
        0,
        0,
        0,
        std::ptr::null(),
    );

    if hwnd == 0 {
        return Err("CreateWindowExW failed".to_string());
    }

    RegisterPowerSettingNotification(hwnd, &DISPLAY_STATE_GUID, DEVICE_NOTIFY_WINDOW_HANDLE);

    Ok(hwnd)
}

// ─── Boucle de messages ────────────────────────────────────────────────────────

/// Exécute la boucle de messages Win32 jusqu'à réception de `WM_QUIT`.
///
/// Bloque le thread appelant. Retourne quand `PostQuitMessage` est appelé
/// (typiquement depuis [`wnd_proc`] sur `WM_DESTROY`, ou après fermeture de la fenêtre).
///
/// # Safety
/// Doit être appelée depuis le même thread que [`create_power_window`].
pub unsafe fn run_message_loop() {
    let mut msg = MSG {
        hwnd: 0,
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: windows_sys::Win32::Foundation::POINT { x: 0, y: 0 },
    };
    while GetMessageW(&mut msg, 0, 0, 0) != 0 {
        DispatchMessageW(&msg);
    }
}
