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

//////////////////////////////////////////////////////////////////////

pub async fn ct_html() -> impl IntoResponse {
  Html(r#"<h1>Hello from html</h1>"#)
}

pub async fn ct_json() -> impl IntoResponse {
  Json(vec!["satu", "dua", "tiga"])
}

pub async fn ct_validation(Json(input): Json<NameInput>) -> impl IntoResponse {
  let Ok(_) = input.validate() else {
    return (StatusCode::BAD_REQUEST, "invalid argument".to_string());
  };

  (StatusCode::OK, format!("your input name: {}", input.name))
}

pub async fn ct_form(Form(input): Form<NameInput>) -> impl IntoResponse {
  (StatusCode::OK, format!("your input name: {}", input.name))
}

//////////////////////////////////////////////////////////////////////

pub async fn crud_list() -> impl IntoResponse {
  format!("Hello crud list")
}

pub async fn crud_read(Path(id): Path<usize>) -> impl IntoResponse {
  format!("Hello read todo({})", id)
}

pub async fn crud_create(Json(input): Json<NameInput>) -> impl IntoResponse {
  format!("Hello create todo({})", input.name)
}

pub async fn crud_update(Path(id): Path<usize>, Json(input): Json<NameInput>) -> impl IntoResponse {
  format!("Hello update todo({},{})", id, input.name)
}

pub async fn crud_partial_update(
  Path(id): Path<usize>,
  Json(input): Json<NameInput>,
) -> impl IntoResponse {
  format!("Hello update todo({},{})", id, input.name)
}

pub async fn crud_delete(Path(id): Path<usize>) -> impl IntoResponse {
  format!("Hello delete todo({})", id)
}

//////////////////////////////////////////////////////////////////////

pub async fn graceful_shutdown() -> impl IntoResponse {
  sleep(Duration::from_secs(5)).await;
  "server graceful shutdown"
}

pub async fn with_rejection(
  // The second constructor argument is not meaningful and can be safely ignored
  WithRejection(Json(_value), _): WithRejection<Json<NameInput>, ApiError>,
) -> impl IntoResponse {
  Json(vec!["satu", "dua", "tiga"])
}

//////////////////////////////////////////////////////////////////////

pub async fn fallback() -> impl IntoResponse {
  Json(HashMap::from([(
    "message".to_string(),
    "route not found".to_string(),
  )]))
}
