use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
  CorsLayer::new()
    .allow_origin("*".parse::<HeaderValue>().unwrap())
    .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
    .allow_methods([
      Method::GET,
      Method::POST,
      Method::PUT,
      Method::PATCH,
      Method::DELETE,
      Method::HEAD,
      Method::OPTIONS,
    ])
}
