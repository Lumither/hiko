use std::process::exit;

use axum::response::Html;
use axum::{routing::get, Router};

use crate::config::Config;
use crate::database::tasks::TaskDB;
use crate::database::Database;
use crate::mail::Mailer;

pub async fn run(config: Config) {
    // Database init
    log::info!("Tasks Database loading");
    let task_db = match TaskDB::connect(
        config.database.url,
        config.database.user,
        config.database.password,
    )
    .await
    {
        Ok(task_db) => task_db,
        Err(e) => {
            log::error!("Tasks database init failed: {}", e.to_string());
            exit(1)
        }
    };
    match task_db.init().await {
        Ok(_) => {
            log::info!("Tasks database initialized")
        }
        Err(e) => {
            log::error!("{}", e);
            exit(1)
        }
    }
    log::info!("Tasks Database loaded");

    // load mail module
    log::info!("Mailer loading");
    let mail = Mailer::new(config.mail);
    log::info!("Mailer loaded");

    // api listen
    let app = Router::new()
        .route("/", get(handler))
        .route("/api", get(create));
    let listener =
        match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.general.port)).await {
            Ok(listener) => {
                log::info!("Server started at port {}", config.general.port);
                listener
            }
            Err(e) => {
                log::error!("TcpListener init failed: {}", e.to_string());
                exit(1)
            }
        };

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("{}", e.to_string());
            exit(1)
        }
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn create() -> Html<&'static str> {
    Html("<h1> Created </h1>")
}
