use crate::config::Config;
use crate::modes::execute_step;
use log::{info, warn};

fn log_step(step: &str, ok: bool) {
    if ok { info!("{} ... ok", step); } else { warn!("{} ... FAIL", step); }
}

pub fn run(config: &Config) {
    for step in &config.pipeline.desktop {
        let ok = execute_step(step, config, false);
        log_step(step, ok);
    }
}
