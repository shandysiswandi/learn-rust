mod controller;
mod error;
mod middleware;
pub mod model;

use axum::{routing, Router};
use controller as h;
use tower_http::trace::TraceLayer;

pub fn init() -> Router {
  let root = Router::new()
    .route("/:name", routing::get(h::path_variable))
    .route("/status-code", routing::get(h::status_code))
    .route("/html", routing::get(h::html))
    .route("/json", routing::get(h::json))
    .route("/reject", routing::post(h::with_rejection))
    .route("/graceful-shutdown", routing::get(h::graceful_shutdown))
    .route("/form", routing::post(h::form))
    .route("/validation", routing::post(h::validation));

  Router::new()
    .nest("/", root)
    .layer(middleware::cors())
    .layer(TraceLayer::new_for_http())
    .fallback(h::fallback)
}
