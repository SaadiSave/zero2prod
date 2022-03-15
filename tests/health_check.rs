#![warn(clippy::pedantic)]

use std::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let resp = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let addr = listener.local_addr().unwrap();

    let server = zero2prod::run(listener).expect("Failed to bind to address");

    tokio::spawn(server);

    addr.to_string()
}
