use crate::domain::user::{commands::create_user::CreateUserCommand, entities::user::User};
use std::error::Error;

pub trait DrivingPortUser: Send + Sync {
    fn create_user(&self, create_user_command: &CreateUserCommand) -> Result<User, Box<dyn Error>>;
}
