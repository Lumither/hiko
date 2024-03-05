use std::process::exit;
use std::sync::Arc;

use axum::response::Html;
use axum::{routing::get, Router};

use crate::config::Config;
use crate::database::tasks::TaskDB;

pub async fn run(config: Config, task_db: Arc<TaskDB>) {
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
