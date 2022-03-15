#![warn(clippy::pedantic)]
#![allow(
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use axum::{routing::get, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}

pub fn run(listener: TcpListener) -> hyper::Result<App> {
    let app = Router::new().route("/health_check", get(health_check));

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
