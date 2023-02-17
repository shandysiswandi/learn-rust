#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    rest_axum::app::run().await
}
