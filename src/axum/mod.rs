mod controller;

use axum::{routing, Router};

pub fn init() -> Router {
  let root = Router::new()
    .route("/", routing::get(controller::index))
    .route("/health", routing::get(controller::health))
    .route("/empty", routing::get(controller::unit_tuple));

  Router::new().nest("/", root)
}
