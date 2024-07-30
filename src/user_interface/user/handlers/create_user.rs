use std::sync::{Arc, Mutex};

use crate::{
    domain::user::{commands::create_user::CreateUserCommand, entities::user::User},
    user_interface::AppState,
};

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[debug_handler(state= Arc<Mutex<AppState>>)]
pub async fn create_user_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(create_user_request): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let create_user_command = CreateUserCommand::new(
        create_user_request.name.clone(),
        create_user_request.email.clone(),
        create_user_request.password.clone(),
    );
    if let Ok(user) = state
        .lock()
        .unwrap()
        .user_service
        .lock()
        .unwrap()
        .create_user(&create_user_command)
    {
        return Ok(Json(user));
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
