mod controller;
mod error;
mod middleware;
mod model;

use self::controller as h;
use axum::{routing, Router};
use tower_http::trace::TraceLayer;

pub fn init() -> Router {
  let root = Router::new()
    // reject is custom error when request Extractor for customizing extractor rejections
    .route("/reject", routing::post(h::with_rejection))
    // graceful-shutdown is route to delay 5 second, to test gracefull shutdown is ok
    .route("/graceful-shutdown", routing::get(h::graceful_shutdown));

  // list of endpoint that handle request or response with different content type
  let content_types = Router::new()
    .route("/validation", routing::post(h::ct_validation))
    .route("/form", routing::post(h::ct_form))
    .route("/html", routing::get(h::ct_html))
    .route("/json", routing::get(h::ct_json));

  // list of endpoint crud
  let todos = Router::new()
    .route("/", routing::get(h::crud_list))
    .route("/:id", routing::get(h::crud_read))
    .route("/", routing::post(h::crud_create))
    .route("/:id", routing::put(h::crud_update))
    .route("/:id", routing::patch(h::crud_partial_update))
    .route("/:id", routing::delete(h::crud_delete));

  Router::new()
    .nest("/", root)
    .nest("/crud", todos)
    .nest("/content-types", content_types)
    .layer(middleware::cors())
    .layer(TraceLayer::new_for_http())
    .fallback(h::fallback)
}
