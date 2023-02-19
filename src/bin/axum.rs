#[tokio::main]
async fn main() {
  let app = learn_rust::axum::init();
  let addr = "[::]:8080".parse().unwrap();

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
