use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello rust!")
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use actix_web::{body, http, web};

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for web::Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn index_ok() {
        // Arrange
        let resp = index().await;

        // Act
        let status = resp.status();
        let byte_body = body::to_bytes(resp.into_body()).await.unwrap();

        //  Assert
        assert_eq!(status, http::StatusCode::OK);
        assert_eq!(byte_body.as_str(), "Hello rust!");
    }

    #[actix_web::test]
    async fn health_ok() {
        // Arrange
        let resp = health().await;

        // Act

        //  Assert
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
