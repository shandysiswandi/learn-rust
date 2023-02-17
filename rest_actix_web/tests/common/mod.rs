use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    dotenv::dotenv().expect("Failed to read .env file for integration test");

    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port for integration test");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let server = rest_actix_web::run(listener).expect("Failed run server");
    let _ = tokio::spawn(server);

    TestApp { address }
}
