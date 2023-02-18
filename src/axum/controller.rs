//! return type of controller/handler function is can be anything
//! as long as that type is implement trait `IntoResponse` from `axum::response::IntoResponse`
//!
//! click `IntoResponse` to see all type that implement trait `IntoResponse`
//!
//!

use axum::{http::StatusCode, response::IntoResponse};

pub async fn index() -> &'static str {
  "Hello, World!"
}

pub async fn health() -> StatusCode {
  StatusCode::OK
}

pub async fn unit_tuple() -> impl IntoResponse {
  ()
}
