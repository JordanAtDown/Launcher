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
    log_step("cec::power_on",          modules::cec::power_on(&config.cec));
    log_step("cec::set_source",        modules::cec::set_source(&config.cec));
    log_step("display::set_game",      modules::display::set_game(&config.display));
    log_step("sound::set_game",        modules::sound::set_game(&config.sound));
    log_step("wsl::shutdown",          modules::wsl::shutdown(&config.wsl));
    log_step("hags::enable",           modules::hags::enable());
    log_step("timerresolution::apply", modules::timerresolution::apply(&config.timerresolution));
    log_step("gamebar::uninstall",     modules::gamebar::uninstall(&config.gamebar));
    log_step("gamemode::enable",       modules::gamemode::enable(&config.gamemode));
    log_step("updates::pause",         modules::updates::pause(&config.updates));
    log_step("notifications::disable", modules::notifications::disable(&config.notifications));
    log_step("powerplan::apply",       modules::powerplan::apply(&config.powerplan.game));
    log_step("killist::apply",                    modules::killist::apply(&config.killist));
    log_step("disable_services::disable",         modules::disable_services::disable(&config.disable_services));
    log_step("rtss::apply",            modules::rtss::apply(&config.rtss, config.rtss.game_limit));
    log_step("afterburner::launch",    modules::afterburner::launch(&config.afterburner, config.afterburner.game_profile));
    log_step("startup::launch",        modules::startup::launch_game(&config.startup));
    log_step("steam::launch",          modules::steam::launch(&config.steam));
}
