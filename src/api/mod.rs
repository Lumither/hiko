use std::process::exit;

use axum::response::Html;
use axum::{routing::get, Router};

pub async fn run(port: u16) {
    let app = Router::new().route("/", get(handler));
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => {
            log::info!("Server started at port {}", port);
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
