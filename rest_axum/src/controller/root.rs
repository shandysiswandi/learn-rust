use axum::{http::StatusCode, response::IntoResponse};

pub async fn index() -> &'static str {
    "Hello, axum"
}

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}
