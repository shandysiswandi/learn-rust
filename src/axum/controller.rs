//! return type of controller/handler function is can be anything
//! as long as that type is implement trait `IntoResponse` from `IntoResponse`
//!
//! click `IntoResponse` to see all type that implement trait `IntoResponse`
//!
//!

use super::{error::ApiError, model::NameInput};
use axum::{
  extract::Path,
  http::StatusCode,
  response::{Html, IntoResponse},
  Form, Json,
};
use axum_extra::extract::WithRejection;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use validator::Validate;

pub async fn path_variable(Path(name): Path<String>) -> impl IntoResponse {
  tracing::info!("listening on {}", name);
  format!("Hello {}", name)
}

pub async fn status_code() -> impl IntoResponse {
  StatusCode::OK
}

pub async fn html() -> impl IntoResponse {
  Html(r#"<h1>Hello from html</h1>"#)
}

pub async fn json() -> impl IntoResponse {
  Json(vec!["satu", "dua", "tiga"])
}

pub async fn graceful_shutdown() -> impl IntoResponse {
  sleep(Duration::from_secs(5)).await;
  "server graceful shutdown"
}

pub async fn validation(Json(input): Json<NameInput>) -> impl IntoResponse {
  let Ok(_) = input.validate() else {
    return (StatusCode::BAD_REQUEST, "invalid argument".to_string());
  };

  (StatusCode::OK, format!("your input name: {}", input.name))
}

pub async fn form(Form(input): Form<NameInput>) -> impl IntoResponse {
  (StatusCode::OK, format!("your input name: {}", input.name))
}

pub async fn with_rejection(
  // The second constructor argument is not meaningful and can be safely ignored
  WithRejection(Json(_value), _): WithRejection<Json<NameInput>, ApiError>,
) -> impl IntoResponse {
  Json(vec!["satu", "dua", "tiga"])
}

pub async fn fallback() -> impl IntoResponse {
  Json(HashMap::from([(
    "message".to_string(),
    "route not found".to_string(),
  )]))
}
