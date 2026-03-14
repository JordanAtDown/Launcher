#![windows_subsystem = "windows"]

mod config;
mod modes;
mod modules;

use log::info;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;
use std::path::PathBuf;

fn main() {
    let config = config::load();

    let log_path = config
        .logging
        .path
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
                .join("launcher.log")
        });

    if let Ok(file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let log_config = ConfigBuilder::new()
            .set_time_offset_to_local()
            .unwrap_or_else(|c| c)
            .build();
        let _ = WriteLogger::init(LevelFilter::Info, log_config, file);
    }

    let game_mode = modules::monitor::resolve_mode(&config.monitor);
    info!("mode={}", if game_mode { "game" } else { "desktop" });

    match game_mode {
        true  => modes::game::run(&config),
        false => modes::desktop::run(&config),
    }
}
