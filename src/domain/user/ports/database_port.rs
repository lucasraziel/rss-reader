use crate::domain::user::entities::user::User;
use std::error::Error;

pub trait DatabasePortUser: Send + Sync {
    fn add_user(&self, user: &User, password: &String) -> Result<User, Box<dyn Error>>;
    fn get_user(&self, id: &String) -> Result<Option<User>, Box<dyn Error>>;
    fn get_user_by_email(&self, email: &String) -> Result<Option<User>, Box<dyn Error>>;
    fn get_users(&self, offset: u32, size: u32) -> Result<Vec<User>, Box<dyn Error>>;
    fn update_user(&self, user: User, password: Option<&String>) -> Result<(), Box<dyn Error>>;
    fn get_hashed_password(&self, email: &String) -> Result<Option<String>, Box<dyn Error>>;
}
