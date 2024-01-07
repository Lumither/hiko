use std::process::exit;

use crate::config::Config;

mod api;
pub mod config;
mod database;
pub mod log;
mod mail;
mod plugin;
mod task;

pub async fn run(config_path: String) {
    // load conf file
    log::info!("Loading config file");
    let config = match Config::from(&config_path) {
        Ok(conf) => {
            log::info!("Config loaded");
            conf
        }
        Err(err) => {
            log::error!("{}", err);
            exit(1);
        }
    };

    // load axum
    api::run(config.general.port).await;
}
