mod controller;

use axum::{routing, Router};

pub fn init() -> Router {
  let root = Router::new()
    .route("/str", routing::get(controller::str))
    .route("/string", routing::get(controller::string))
    .route("/status-code", routing::get(controller::status_code))
    .route("/unit-tuple", routing::get(controller::unit_tuple))
    .route("/html", routing::get(controller::html))
    .route("/json", routing::get(controller::json));

  Router::new().nest("/", root)
}
