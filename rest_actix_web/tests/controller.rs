mod common;

use common::*;
use reqwest::{Client, Response};

#[actix_rt::test]
async fn root_index() {
    // Arrange
    let app: TestApp = spawn_app().await;
    let client: Client = Client::new();

    // Act
    let resp: Response = client
        .get(&format!("{}", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    let code: u16 = resp.status().as_u16();
    let body: String = resp.text().await.expect("Failed to decode body");

    // assert
    assert_eq!(code, 200);
    assert_eq!(body, "Hello rust!");
}

#[actix_rt::test]
async fn root_health() {
    // Arrange
    let app: TestApp = spawn_app().await;
    let client: Client = Client::new();

    // Act
    let resp: Response = client
        .get(&format!("{}", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    let code: u16 = resp.status().as_u16();

    // Assert
    assert_eq!(code, 200);
}

#[actix_rt::test]
async fn root_no_route() {
    // Arrange
    let app: TestApp = spawn_app().await;
    let client: Client = Client::new();

    // Act
    let resp: Response = client
        .get(&format!("{}/not-found", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    let code: u16 = resp.status().as_u16();
    let body: String = resp.text().await.expect("Failed to decode body");

    // assert
    assert_eq!(code, 404);
    assert_eq!(body, "{\"message\":\"route not found\"}");
}