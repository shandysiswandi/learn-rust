use axum::{
  extract::rejection::JsonRejection,
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
  // see: https://github.com/tokio-rs/axum/tree/main/examples/customize-extractor-error
  JsonExtractorRejection(JsonRejection),
}

impl From<JsonRejection> for ApiError {
  fn from(value: JsonRejection) -> Self {
    Self::JsonExtractorRejection(value)
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let payload = json!({
        "message": "self",
        "origin": "with_rejection"
    });

    let code = match self {
      ApiError::JsonExtractorRejection(x) => match x {
        JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
        JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
      },
    };

    (code, Json(payload)).into_response()
  }
}
