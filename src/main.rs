use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let app = Router::new().route("/health_check", get(health_check));

    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
        .await
}
