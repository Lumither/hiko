use axum::response::Html;
use axum::{routing::get, Router};

pub async fn run() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("[fatal] tcp listener init failed");
    axum::serve(listener, app)
        .await
        .expect("[fatal] server failed to start");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
