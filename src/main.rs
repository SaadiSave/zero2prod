#![warn(clippy::pedantic)]
#![allow(
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod::{
    config::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let sub = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(sub);

    let conf = get_config().expect("Cannot read config");

    let addr = format!("127.0.0.1:{}", conf.port);

    let pool = PgPool::connect(conf.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to bind to port");

    run(listener, pool)
        .unwrap_or_else(|e| panic!("Application failed to start: {e}"))
        .await
        .unwrap()
}
