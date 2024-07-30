mod application;
mod domain;
mod infrastructure;
mod user_interface;

use application::user::user_service::UserService;
use infrastructure::bd::DatabaseConnection;
use infrastructure::repositories::user_repository::UserRepositoryImpl;
use std::sync::{Arc, Mutex};
use tokio;
use user_interface::{get_app_rest, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_connection = Arc::new(Mutex::new(DatabaseConnection::new()?));
    let user_repository = UserRepositoryImpl::new(database_connection);
    let user_service = UserService::new(Box::new(user_repository));
    let app_state = AppState {
        user_service: Arc::new(Mutex::new(user_service)),
        user_repository: Arc::new(Mutex::new(user_repository)),
    };
    let app_rest = get_app_rest(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app_rest).await.unwrap();

    Ok(())
}
