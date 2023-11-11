use axum::response::Html;
use axum::{routing::get, Router};

pub async fn run() {
    let app = Router::new().route("/", get(handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("[Fatal] axum start failed, killed");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
