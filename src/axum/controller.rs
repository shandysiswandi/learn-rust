//! return type of controller/handler function is can be anything
//! as long as that type is implement trait `IntoResponse` from `axum::response::IntoResponse`
//!
//! click `IntoResponse` to see all type that implement trait `IntoResponse`
//!
//!

use axum::{http::StatusCode, response::Html, Json};

pub async fn str() -> &'static str {
  "Hello from string literal"
}

pub async fn string() -> String {
  "Hello from string".to_owned()
}

pub async fn status_code() -> StatusCode {
  StatusCode::OK
}

pub async fn unit_tuple() -> () {
  ()
}

pub async fn html() -> Html<&'static str> {
  Html(r#"<h1>Hello from html</h1>"#)
}

pub async fn json() -> Json<Vec<&'static str>> {
  Json(vec!["satu", "dua", "tiga"])
}
