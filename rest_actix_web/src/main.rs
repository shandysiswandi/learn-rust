use actix_web::web;
use rest_actix_web::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let host: String = std::env::var("APP_HOST").unwrap_or_default();
    let port: String = std::env::var("APP_PORT").unwrap_or_default();
    let address: String = format!("{host}:{port}");

    println!("server running on {address}");
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .data_factory(app_state_factory)
            .configure(routes)
    })
    .bind(address)?
    .run()
    .await
}

/// to initialize data or depedencies from service repo and other source
///
/// depedency injection goes here
async fn app_state_factory() -> std::io::Result<state::AppState> {
    let db_url: String = std::env::var("DB_URL").unwrap_or_default();
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect(db_url.as_str())
        .await
        .expect("failed to connect database");

    Ok(state::AppState {
        todo_svc: std::sync::Arc::new(service::todo::TodoService::new()),
        db: pool,
    })
}

/// To registered route from controller
///
/// by module
fn routes(svc_config: &mut web::ServiceConfig) {
    svc_config
        .route("/", web::get().to(controller::root::index))
        .route("/health", web::get().to(controller::root::health))
        .route("/todos", web::get().to(controller::todo::todos))
        .route("/todos/{id}", web::get().to(controller::todo::todo_by_id))
        .route("/todos", web::post().to(controller::todo::create_todo))
        .route("/todos/{id}", web::put().to(controller::todo::update_todo))
        .route(
            "/todos/{id}",
            web::delete().to(controller::todo::delete_todo),
        );
}
