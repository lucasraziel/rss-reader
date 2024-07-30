use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: &str, email: &str) -> User {
        User {
            name: name.to_string(),
            email: email.to_string(),
            id: Uuid::new_v4().to_string(),
        }
    }
}
