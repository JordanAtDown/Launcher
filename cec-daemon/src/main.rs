// cec-daemon : garde la connexion CEC ouverte et envoie standby/wake
// à la TV selon l'état de l'écran Windows et les événements d'arrêt.
//
// Usage : cec-daemon.exe --path "<chemin vers cec-client.exe>"
//
// Événements gérés :
//   - GUID_CONSOLE_DISPLAY_STATE OFF → standby TV  (couvre S3, S0, inactivité)
//   - GUID_CONSOLE_DISPLAY_STATE ON  → allume TV + source active
//   - CTRL_SHUTDOWN_EVENT / CTRL_LOGOFF_EVENT → standby TV

#![allow(non_snake_case)]

use std::ffi::OsStr;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::Console::{
    SetConsoleCtrlHandler, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
};
use windows_sys::Win32::System::Power::RegisterPowerSettingNotification;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
    RegisterClassExW, CW_USEDEFAULT, DEVICE_NOTIFY_WINDOW_HANDLE, MSG, PBT_POWERSETTINGCHANGE,
    WM_DESTROY, WM_POWERBROADCAST, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// GUID_CONSOLE_DISPLAY_STATE {6FE69556-704A-47A0-8F24-C28D936FDA47}
// Envoyé via WM_POWERBROADCAST/PBT_POWERSETTINGCHANGE quand l'écran s'allume/s'éteint.
// lParam → [GUID: 16 bytes][DataLength: 4 bytes][Data: DataLength bytes]
// Data[0..4] = u32 : 0=OFF, 1=ON, 2=Dimmed
const DISPLAY_STATE_GUID: GUID = GUID {
    data1: 0x6FE6_9556,
    data2: 0x704A,
    data3: 0x47A0,
    data4: [0x8F, 0x24, 0xC2, 0x8D, 0x93, 0x6F, 0xDA, 0x47],
};

const DISPLAY_OFF: u32 = 0;
const DISPLAY_ON: u32 = 1;

static CEC_STDIN: std::sync::OnceLock<Arc<Mutex<std::process::ChildStdin>>> =
    std::sync::OnceLock::new();

fn send_cec(cmd: &str) {
    if let Some(arc) = CEC_STDIN.get() {
        if let Ok(mut s) = arc.lock() {
            let _ = write!(s, "{}\n", cmd);
            let _ = s.flush();
            log::info!("cec-daemon: sent '{}'", cmd);
        }
    }
}

unsafe extern "system" fn ctrl_handler(ctrl_type: u32) -> i32 {
    match ctrl_type {
        CTRL_SHUTDOWN_EVENT | CTRL_LOGOFF_EVENT => {
            log::info!("cec-daemon: shutdown/logoff → standby");
            send_cec("standby 0");
            std::thread::sleep(std::time::Duration::from_millis(500));
            0
        }
        _ => 0,
    }
}

fn wstr(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Compare deux GUIDs octet par octet.
unsafe fn guid_eq(a: &GUID, b: &GUID) -> bool {
    std::slice::from_raw_parts(a as *const GUID as *const u8, 16)
        == std::slice::from_raw_parts(b as *const GUID as *const u8, 16)
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_POWERBROADCAST && wparam as u32 == PBT_POWERSETTINGCHANGE && lparam != 0 {
        // lParam → [GUID: 16 bytes][DataLength: u32: 4 bytes][Data: ...]
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
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        send_cec("as");
                    }
                    _ => {}
                }
            }
        }
    } else if msg == WM_DESTROY {
        PostQuitMessage(0);
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn setup_logging() {
    let log_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("cec-daemon.log");
    let _ = simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("cannot open cec-daemon.log"),
    );
}

fn spawn_cec_client(path: &str) -> Child {
    log::info!("cec-daemon: spawning cec-client: {}", path);
    Command::new(path)
        .arg("-s")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| {
            log::warn!("cec-daemon: cannot spawn cec-client: {}", e);
            std::process::exit(1);
        })
}

fn main() {
    setup_logging();

    let args: Vec<String> = std::env::args().collect();
    let cec_path = args
        .windows(2)
        .find(|w| w[0] == "--path")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| {
            eprintln!("usage: cec-daemon.exe --path <cec-client.exe>");
            std::process::exit(1);
        });

    let mut child = spawn_cec_client(&cec_path);
    let stdin = child.stdin.take().expect("no stdin");
    CEC_STDIN
        .set(Arc::new(Mutex::new(stdin)))
        .expect("OnceLock already set");

    log::info!("cec-daemon: started, cec-client pid={}", child.id());

    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_handler), 1);

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
            log::warn!("cec-daemon: CreateWindowExW failed");
            std::process::exit(1);
        }

        RegisterPowerSettingNotification(
            hwnd,
            &DISPLAY_STATE_GUID,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        );

        log::info!("cec-daemon: listening for display/shutdown events");

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

    send_cec("standby 0");
    std::thread::sleep(std::time::Duration::from_millis(300));
    send_cec("q");
    let _ = child.wait();
}
