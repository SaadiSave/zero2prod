use axum::{
    extract::Extension,
    http::Request,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use uuid::Uuid;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

// from https://docs.rs/tower-http/0.2.5/tower_http/request_id/index.html#using-uuids
#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/subscriptions", post(crate::routes::subscribe))
        .layer(Extension(pool))
        .layer(
            // from https://docs.rs/tower-http/0.2.5/tower_http/request_id/index.html#using-trace
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                ),
        );

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
