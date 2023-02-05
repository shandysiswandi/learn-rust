use crate::model::api::ErrorResponseJson;
use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello rust!")
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn no_route() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponseJson {
        message: "route not found".to_string(),
    })
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use actix_web::{body, http, web};

    #[actix_web::test]
    async fn index_ok() {
        // Arrange
        let resp = index().await;

        // Act
        let status = resp.status();
        let byte_body = body::to_bytes(resp.into_body()).await.unwrap();

        //  Assert
        assert_eq!(status, http::StatusCode::OK);
        assert_eq!(byte_body, web::Bytes::from("Hello rust!"));
    }

    #[actix_web::test]
    async fn health_ok() {
        // Arrange
        let resp = health().await;

        // Act

        //  Assert
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn no_route_ok() {
        // Arrange
        let resp = no_route().await;

        // Act
        let expect_status = resp.status();
        let bytes = body::to_bytes(resp.into_body()).await.unwrap();
        let expect_body = std::str::from_utf8(&bytes).unwrap();

        //  Assert
        assert_eq!(expect_status, http::StatusCode::NOT_FOUND);
        assert_eq!(expect_body, "{\"message\":\"route not found\"}");
    }
}
