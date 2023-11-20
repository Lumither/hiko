use crate::config::Config;

mod api;
pub mod config;
mod database;
pub mod log;
mod plugin;
mod task;

pub async fn run(config_path: String) {
    // load conf file
    log::info!("Loading config file");
    let _config = match Config::from(&config_path) {
        Ok(conf) => {
            log::info!("Config loaded");
            conf
        }
        Err(err) => {
            log::error!("{}", err);
            panic!() // todo: better stop
        }
    };

    //load axum
    api::run().await;
}
