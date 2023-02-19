mod controller;
use controller as h;

use axum::{routing, Router};

pub fn init() -> Router {
  let root = Router::new()
    .route("/str", routing::get(h::str))
    .route("/string", routing::get(h::string))
    .route("/status-code", routing::get(h::status_code))
    .route("/unit-tuple", routing::get(h::unit_tuple))
    .route("/html", routing::get(h::html))
    .route("/graceful-shutdown", routing::get(h::graceful_shutdown))
    .route("/json", routing::get(h::json));

  Router::new().nest("/", root).fallback(h::fallback)
}
