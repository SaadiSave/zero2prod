#![warn(clippy::pedantic)]
#![allow(
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    run(TcpListener::bind("127.0.0.1:8000").unwrap())?.await
}
