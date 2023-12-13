use axum::response::Html;
use axum::{routing::get, Router};
use std::process::exit;

pub async fn run() {
    let app = Router::new().route("/", get(handler));
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("TcpListener init failed: {}", e.to_string());
            exit(1)
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => log::info!("Server started"),
        Err(e) => {
            log::error!("{}", e.to_string());
            exit(1)
        }
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
