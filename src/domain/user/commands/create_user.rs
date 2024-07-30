pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateUserCommand {
    pub fn new(name: String, email: String, password: String) -> Self {
        CreateUserCommand {
            name,
            email,
            password,
        }
    }
}
