use crate::AppState;
use actix_web::{web, HttpResponse};

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello rust!")
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn fetch_notification(_deps: web::Data<AppState>) -> HttpResponse {
    todo!()
}

pub async fn find_notification(_deps: web::Data<AppState>) -> HttpResponse {
    todo!()
}

pub async fn count_unread_notification(_deps: web::Data<AppState>) -> HttpResponse {
    todo!()
}

pub async fn read_notification(_deps: web::Data<AppState>) -> HttpResponse {
    todo!()
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
}
