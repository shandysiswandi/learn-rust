use crate::controller::root;
use axum::{routing, Router};

pub async fn run() {
    let addr = "[::]:8080".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

fn routes() -> Router {
    let root = Router::new()
        .route("/", routing::get(root::index))
        .route("/health", routing::get(root::health));

    let todo = Router::new()
        .route("/", routing::get(root::index))
        .route("/:id", routing::get(root::health))
        .route("/", routing::post(root::index))
        .route("/:id", routing::put(root::index))
        .route("/:id", routing::patch(root::index))
        .route("/:id", routing::delete(root::index));

    Router::new().nest("/", root).nest("/todo", todo)
}
