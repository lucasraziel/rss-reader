use crate::domain::user::commands::create_user::CreateUserCommand;
use crate::domain::user::entities::{auth::Auth, user::User};
use crate::domain::user::ports::{database_port::DatabasePortUser, driving_port::DrivingPortUser};
use std::error::Error;

pub struct UserService {
    database_port_user: Box<dyn DatabasePortUser>,
}

impl UserService {
    pub fn new(database_port: Box<dyn DatabasePortUser>) -> Self {
        UserService {
            database_port_user: database_port,
        }
    }
}

impl DrivingPortUser for UserService {
    fn create_user(&self, command: &CreateUserCommand) -> Result<User, Box<dyn Error>> {
        let user = User::new(&command.name, &command.email);
        let auth = Auth::new(&command.password, &user.id);

        self.database_port_user
            .add_user(&user, &auth.password_hashed)
    }
}
