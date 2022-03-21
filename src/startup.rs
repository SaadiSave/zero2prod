use axum::{
    extract::Extension,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/subscriptions", post(crate::routes::subscribe))
        .layer(Extension(pool));

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
