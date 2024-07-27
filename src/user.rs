#[derive(Debug)]
pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: &str, email: &str) -> User {
        User {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}
