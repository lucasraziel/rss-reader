pub mod user_repository;

use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

impl User {
    pub fn new(name: &str, email: &str) -> User {
        User {
            name: name.to_string(),
            email: email.to_string(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
