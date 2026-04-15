pub mod desktop;
pub mod game;

use crate::config::Config;
use crate::modules;

pub fn execute_step(step: &str, config: &Config, is_game: bool) -> bool {
    match step {
        "cec::power_on"             => modules::cec::power_on(&config.cec),
        "cec::set_source"           => modules::cec::set_source(&config.cec),
        "cec::launch_daemon"        => modules::cec::launch_daemon(&config.cec),
        "display::set_game"         => modules::display::set_game(&config.display),
        "display::set_desktop"      => modules::display::set_desktop(&config.display),
        "sound::set_game"           => modules::sound::set_game(&config.sound),
        "sound::set_desktop"        => modules::sound::set_desktop(&config.sound),
        "wsl::shutdown"             => modules::wsl::shutdown(&config.wsl),
        "hags::enable"              => modules::hags::enable(),
        "hags::disable"             => modules::hags::disable(),
        "timerresolution::apply"    => modules::timerresolution::apply(&config.timerresolution),
        "gamebar::uninstall"        => modules::gamebar::uninstall(&config.gamebar),
        "gamemode::enable"          => modules::gamemode::enable(&config.gamemode),
        "updates::pause"            => modules::updates::pause(&config.updates),
        "updates::restore"          => modules::updates::restore(&config.updates),
        "notifications::disable"    => modules::notifications::disable(&config.notifications),
        "notifications::restore"    => modules::notifications::restore(&config.notifications),
        "killist::apply"            => modules::killist::apply(&config.killist),
        "disable_services::disable" => modules::disable_services::disable(&config.disable_services),
        "disable_services::restore" => modules::disable_services::restore(&config.disable_services),
        "steam::launch"             => modules::steam::launch(&config.steam),
        "powerplan::apply" => {
            let plan = if is_game { &config.powerplan.game } else { &config.powerplan.desktop };
            modules::powerplan::apply(plan)
        }
        "rtss::apply" => {
            let limit = if is_game { config.rtss.game_limit } else { None };
            modules::rtss::apply(&config.rtss, limit)
        }
        "afterburner::launch" => {
            let profile = if is_game { config.afterburner.game_profile } else { config.afterburner.desktop_profile };
            modules::afterburner::launch(&config.afterburner, profile)
        }
        "startup::launch" => {
            if is_game { modules::startup::launch_game(&config.startup) }
            else       { modules::startup::launch_desktop(&config.startup) }
        }
        _ => { log::warn!("pipeline: step inconnu '{}'", step); false }
    }
}
