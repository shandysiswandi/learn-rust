pub mod controller;

//
use actix_web::{dev::Server, web, App, HttpServer};
use notification::{port::Usecase, usecase::UsecaseImpl};
use repo_sqlx::{Connection, ConnectionDriver};
use std::{env, io::Result, net::TcpListener, sync::Arc};

// app state using for depedency injection on contreoller
#[derive(Clone)]
pub struct AppState {
    pub svc: Arc<dyn Usecase>,
}

pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .data_factory(app_state_factory)
            .route("/", web::get().to(controller::index))
            .route("/health", web::get().to(controller::health))
            .route(
                "/notifications",
                web::get().to(controller::fetch_notification),
            )
            .route(
                "/notifications/{id}",
                web::get().to(controller::find_notification),
            )
            .route(
                "/notifications/count-unread",
                web::get().to(controller::count_unread_notification),
            )
            .route(
                "/notifications/{id}/read",
                web::post().to(controller::read_notification),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn app_state_factory() -> Result<AppState> {
    let dsn: String = env::var("DB_URL").unwrap_or_default();

    let repo = Connection::new(ConnectionDriver::Mysql, dsn.as_str())
        .connect()
        .await
        .unwrap();
    let svc = UsecaseImpl::new(repo);

    Ok(AppState { svc: Arc::new(svc) })
}
