pub mod handlers;

use std::sync::{Arc, Mutex};

use axum::{routing::post, Router};
use handlers::create_user::create_user_handler;

use super::AppState;

pub fn route_users() -> Router<Arc<Mutex<AppState>>> {
    Router::new().route("/", post(create_user_handler))
}
