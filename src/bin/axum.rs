use learn_rust::axum as appaxum;

#[tokio::main]
async fn main() {
  let app = appaxum::init();
  let addr = "[::]:8080".parse().unwrap();

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
