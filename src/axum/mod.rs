mod controller;
mod middleware;
pub mod model;

use axum::{routing, Router};
use controller as h;

pub fn init() -> Router {
  let root = Router::new()
    .route("/:name", routing::get(h::path_variable))
    .route("/status-code", routing::get(h::status_code))
    .route("/html", routing::get(h::html))
    .route("/json", routing::get(h::json))
    .route("/graceful-shutdown", routing::get(h::graceful_shutdown))
    .route("/validation", routing::post(h::validation));

  Router::new()
    .nest("/", root)
    .layer(middleware::cors())
    .fallback(h::fallback)
}
