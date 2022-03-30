#![warn(clippy::pedantic)]

use hyper::StatusCode;
use once_cell::sync::Lazy;
use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::{SocketAddr, TcpListener};
use zero2prod::{
    config::{get_config, DbSettings},
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "test".to_string();
    let level = "debug".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let sub = get_subscriber(name, level, std::io::stdout);
        init_subscriber(sub);
    } else {
        let sub = get_subscriber(name, level, std::io::sink);
        init_subscriber(sub);
    }
});

struct TestApp {
    addr: SocketAddr,
    pool: PgPool,
}

async fn config_db(config: &DbSettings) -> PgPool {
    let mut conn = PgConnection::connect(config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to database");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Unable to create database");

    let pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate a database");

    pool
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap();

    let mut conf = get_config().expect("Failed to read config");
    conf.database.db_name = uuid::Uuid::new_v4().to_string();
    let pool = config_db(&conf.database).await;

    let server =
        zero2prod::startup::run(listener, pool.clone()).expect("Failed to bind to address");

    tokio::spawn(server);

    TestApp { addr, pool }
}

#[tokio::test]
async fn health_check_works() {
    let TestApp { addr, .. } = spawn_app().await;

    let client = Client::new();

    let resp = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let TestApp { addr, pool } = spawn_app().await;

    let client = Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let resp = client
        .post(format!("http://{addr}/subscriptions"))
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .await
        .expect("Request failed");

    assert_eq!(resp.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_422_for_invalid_data() {
    let TestApp { addr, .. } = spawn_app().await;

    let cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    let client = Client::new();

    for (invalid_body, msg) in cases {
        let resp = client
            .post(format!("http://{addr}/subscriptions"))
            .header(
                hyper::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(invalid_body)
            .send()
            .await
            .expect("Request failed");

        assert_eq!(
            resp.status(),
            StatusCode::UNPROCESSABLE_ENTITY, // Axum defaults to 422 instead of 400
            "The API did not fail with 422 Unprocessable Entity when the payload was {msg}."
        );
    }
}
