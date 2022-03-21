use axum::extract::{Extension, Form};
use hyper::StatusCode;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<FormData>,
) -> StatusCode {
    if sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        time::OffsetDateTime::now_utc()
    )
    .execute(&pool)
    .await
    .is_ok()
    {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
