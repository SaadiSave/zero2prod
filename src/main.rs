#![warn(clippy::pedantic)]
#![allow(
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{config::get_config, startup::run};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let conf = get_config().expect("Cannot read config");

    let addr = format!("127.0.0.1:{}", conf.port);

    let pool = PgPool::connect(&conf.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(addr).expect("Unable to bind to port");

    run(listener, pool)?.await
}
