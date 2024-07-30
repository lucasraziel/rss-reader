use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct Auth {
    pub password_hashed: String,
    pub user_id: String,
}

impl Auth {
    pub fn new(password: &str, user_id: &str) -> Auth {
        Auth {
            password_hashed: hash_password(password),
            user_id: user_id.to_string(),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(&self.password_hashed).unwrap();
        argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
