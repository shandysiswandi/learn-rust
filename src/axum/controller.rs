//! return type of controller/handler function is can be anything
//! as long as that type is implement trait `IntoResponse` from `IntoResponse`
//!
//! click `IntoResponse` to see all type that implement trait `IntoResponse`
//!
//!

use super::model::NameInput;
use axum::{http::StatusCode, response::Html, Json};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use validator::Validate;

pub async fn str() -> &'static str {
  "Hello from string literal"
}

pub async fn string() -> String {
  "Hello from string".to_owned()
}

pub async fn unit_tuple() {}

pub async fn status_code() -> StatusCode {
  StatusCode::OK
}

pub async fn html() -> Html<&'static str> {
  Html(r#"<h1>Hello from html</h1>"#)
}

pub async fn json() -> Json<Vec<&'static str>> {
  Json(vec!["satu", "dua", "tiga"])
}

pub async fn graceful_shutdown() -> &'static str {
  sleep(Duration::from_secs(5)).await;
  "server graceful shutdown"
}

pub async fn validation(Json(input): Json<NameInput>) -> (StatusCode, String) {
  let Ok(_) = input.validate() else {
    return (StatusCode::BAD_REQUEST, "invalid argument".to_string());
  };

  (StatusCode::OK, format!("your input name: {}", input.name))
}

pub async fn fallback() -> Json<HashMap<String, String>> {
  Json(HashMap::from([(
    "message".to_string(),
    "route not found".to_string(),
  )]))
}
