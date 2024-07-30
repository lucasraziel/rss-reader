pub mod user;

use std::sync::{Arc, Mutex};

use crate::domain::user::ports::database_port::DatabasePortUser;
use crate::domain::user::ports::driving_port::DrivingPortUser;
use crate::user_interface::user::route_users;
use axum::Router;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<Mutex<dyn DrivingPortUser>>,
    pub user_repository: Arc<Mutex<dyn DatabasePortUser>>,
}

pub fn get_app_rest(app_state: AppState) -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .nest("/users", route_users())
        .with_state(Arc::new(Mutex::new(app_state)))
}
