#[derive(Clone)]
pub struct AppState {
    pub todo_svc: std::sync::Arc<dyn crate::service::todo::TodoPort>,
    pub db: sqlx::MySqlPool,
}
