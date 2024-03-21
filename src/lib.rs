use std::process::exit;
use std::sync::Arc;

use crate::config::Config;
use crate::database::record::RecordDB;
use crate::database::tasks::TaskDB;
use crate::database::Database;

mod api;
pub mod config;
mod database;
pub mod log;
mod mail;
mod plugin;
mod task;
mod utils;
mod worker;

pub async fn run(config_path: String) {
    // load conf file
    println!("Loading config file");
    let config = match Config::from(&config_path) {
        Ok(conf) => {
            println!("Config loaded");
            conf
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    // log init
    log::init(config.general.log_path.clone());

    // Task Database init
    log::info!("Tasks Database loading");
    let tasks_database = Arc::new(
        TaskDB::connect(
            &config.database.url,
            &config.database.user,
            &config.database.password,
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("Tasks Database init failed: {}", e.to_string());
            exit(1)
        }),
    );
    match tasks_database.init().await {
        Ok(_) => {
            log::info!("Tasks Database initialized")
        }
        Err(e) => {
            log::error!("{}", e);
            exit(1)
        }
    }
    log::info!("Tasks Database loaded");

    // Records Database init
    log::info!("Records Database loading");
    let records_database = Arc::new(
        RecordDB::connect(
            &config.database.url,
            &config.database.user,
            &config.database.password,
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("Records Database init failed: {}", e.to_string());
            exit(1)
        }),
    );
    match records_database.init().await {
        Ok(_) => {
            log::info!("Records Database initialized")
        }
        Err(e) => {
            log::error!("{}", e);
            exit(1)
        }
    }
    log::info!("Records Database loaded");

    // start services
    let worker_thread = tokio::spawn(worker::run(
        config.clone(),
        tasks_database.clone(),
        records_database.clone(),
    ));
    let api_thread = tokio::spawn(api::run(config.clone(), tasks_database.clone()));
    let _ = tokio::join!(worker_thread, api_thread);
}
