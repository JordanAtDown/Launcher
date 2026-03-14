pub mod afterburner;
pub mod cec;
pub mod disable_services;
pub mod display;
pub mod gamebar;
pub mod hags;
pub mod gamemode;
pub mod killist;
pub mod monitor;
pub mod notifications;
pub mod powerplan;
pub mod rtss;
pub mod sound;
pub mod startup;
pub mod steam;
pub mod timerresolution;
pub mod updates;
pub mod wsl;

use std::ffi::c_void;

type HANDLE  = *mut c_void;
type BOOL    = i32;
type DWORD   = u32;
type WORD    = u16;
type LPWSTR  = *mut u16;

const STARTF_USESHOWWINDOW: DWORD = 0x0000_0001;
const SW_SHOWMINIMIZED:     WORD  = 2;
const CREATE_NO_WINDOW:     DWORD = 0x0800_0000;

#[repr(C)]
struct STARTUPINFOW {
    cb:              DWORD,
    lp_reserved:     *const u16,
    lp_desktop:      *const u16,
    lp_title:        *const u16,
    dw_x:            DWORD,
    dw_y:            DWORD,
    dw_x_size:       DWORD,
    dw_y_size:       DWORD,
    dw_x_count_chars: DWORD,
    dw_y_count_chars: DWORD,
    dw_fill_attr:    DWORD,
    dw_flags:        DWORD,
    w_show_window:   WORD,
    cb_reserved2:    WORD,
    lp_reserved2:    *mut u8,
    h_std_input:     HANDLE,
    h_std_output:    HANDLE,
    h_std_error:     HANDLE,
}

#[repr(C)]
struct PROCESS_INFORMATION {
    h_process:    HANDLE,
    h_thread:     HANDLE,
    dw_process_id: DWORD,
    dw_thread_id:  DWORD,
}

extern "system" {
    fn CreateProcessW(
        lp_app: *const u16,
        lp_cmd: LPWSTR,
        lp_proc_attr: *const c_void,
        lp_thread_attr: *const c_void,
        b_inherit: BOOL,
        dw_flags: DWORD,
        lp_env: *const c_void,
        lp_cur_dir: *const u16,
        lp_si: *const STARTUPINFOW,
        lp_pi: *mut PROCESS_INFORMATION,
    ) -> BOOL;
    fn CloseHandle(h: HANDLE) -> BOOL;
}

/// Spawne un processus GUI en démarrage réduit (minimisé dans la barre des tâches/tray).
/// Retourne le PID en cas de succès, None en cas d'erreur.
pub(super) fn spawn_minimized(path: &str, args: &[&str]) -> Option<u32> {
    let mut cmdline = format!("\"{}\"", path);
    for arg in args {
        cmdline.push(' ');
        cmdline.push_str(arg);
    }
    let mut cmdline_w: Vec<u16> = cmdline.encode_utf16().collect();
    cmdline_w.push(0);

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    si.dw_flags = STARTF_USESHOWWINDOW;
    si.w_show_window = SW_SHOWMINIMIZED;

    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let ok = unsafe {
        CreateProcessW(
            std::ptr::null(),
            cmdline_w.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            0,
            CREATE_NO_WINDOW,
            std::ptr::null(),
            std::ptr::null(),
            &si,
            &mut pi,
        )
    };

    if ok != 0 {
        let pid = pi.dw_process_id;
        unsafe { CloseHandle(pi.h_process); CloseHandle(pi.h_thread); }
        Some(pid)
    } else {
        None
    }
}
