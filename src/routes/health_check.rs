use hyper::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
