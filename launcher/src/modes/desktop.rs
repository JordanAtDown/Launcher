use crate::config::Config;
use crate::modules;
use log::{info, warn};

fn log_step(step: &str, ok: bool) {
    if ok {
        info!("{} ... ok", step);
    } else {
        warn!("{} ... FAIL", step);
    }
}

pub fn run(config: &Config) {
    log_step("display::set_desktop",              modules::display::set_desktop(&config.display));
    log_step("disable_services::restore",         modules::disable_services::restore(&config.disable_services));
    log_step("sound::set_desktop",               modules::sound::set_desktop(&config.sound));
    log_step("hags::disable",       modules::hags::disable());
    log_step("powerplan::apply",    modules::powerplan::apply(&config.powerplan.desktop));
    log_step("afterburner::launch", modules::afterburner::launch(&config.afterburner, config.afterburner.desktop_profile));
    log_step("startup::launch",     modules::startup::launch_desktop(&config.startup));
}
