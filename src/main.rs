mod bd;
mod hash;
mod user;

use bd::DatabaseConnection;
use hash::hash_password;
use std::sync::{Arc, Mutex};
use user::user_repository::{UserRepository, UserRepositoryImpl};
use user::User;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new("Lucas", "email");
    println!("Creating users with id: {}", user.get_id());
    let db = DatabaseConnection::new()?;
    println!("Connected to database");
    let user_repository = UserRepositoryImpl::new(Arc::new(Mutex::new(db)));

    let password = hash_password(&"password".to_string());
    println!("Hashed password with length: {}", password.len());

    user_repository.add_user(&user, &password)?;

    println!("User added");

    let user = user_repository.get_user(&user.get_id().to_string())?.unwrap();

    println!("User created with id: {}", user.get_id());

    Ok(())
}
