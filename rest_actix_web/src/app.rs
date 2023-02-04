use crate::{
    controller::{root, todo},
    repository::todo::TodoMysql,
    service::todo::TodoSvc,
    traits::traits::TodoService,
};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, io::Result, net::TcpListener, sync::Arc};

// app state using for depedency injection on contreoller
#[derive(Clone)]
pub struct AppState {
    pub todo_svc: Arc<dyn TodoService>,
}

pub fn run(listener: TcpListener) -> Result<Server> {
    let server =
        HttpServer::new(move || App::new().data_factory(app_state_factory).configure(routes))
            .listen(listener)?
            .run();

    Ok(server)
}

pub async fn mysql_conn(dsn: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .connect(dsn)
        .await
        .expect("failed to connect database")
}

async fn app_state_factory() -> Result<AppState> {
    let dsn: String = env::var("DB_URL").unwrap_or_default();
    let pool: MySqlPool = mysql_conn(dsn.as_str()).await;

    let todo_repo = TodoMysql::new(pool);
    let todo_svc = TodoSvc::new(Box::new(todo_repo));

    Ok(AppState {
        todo_svc: Arc::new(todo_svc),
    })
}

fn routes(svc_config: &mut web::ServiceConfig) {
    svc_config
        .route("/", web::get().to(root::index))
        .route("/health", web::get().to(root::health))
        .route("/todos", web::get().to(todo::todos))
        .route("/todos/{id}", web::get().to(todo::todo_by_id))
        .route("/todos", web::post().to(todo::create_todo))
        .route("/todos/{id}", web::put().to(todo::update_todo))
        .route("/todos/{id}", web::delete().to(todo::delete_todo));
}
