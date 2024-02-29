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
    println!("Loading config file");
    let config = match Config::from(&config_path) {
        Ok(conf) => {
            println!("Config loaded");
            conf
        }
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    log::init(config.general.log_path.clone());

    // load axum
    api::run(config).await;
}
