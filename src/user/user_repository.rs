use crate::user::User;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::bd::DatabaseConnection;

pub trait UserRepository {
    fn add_user(&self, user: &User, password: &String) -> Result<(), Box<dyn Error>>;
    fn get_user(&self, id: &String) -> Result<Option<User>, Box<dyn Error>>;
    fn get_user_by_email(&self, email: &String) -> Result<Option<User>, Box<dyn Error>>;
    fn get_users(&self, offset: u32, size: u32) -> Result<Vec<User>, Box<dyn Error>>;
    fn update_user(
        &self,
        user: User,
        password: Option<&String>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_hashed_password(&self, email: &String) -> Result<Option<String>, Box<dyn Error>>;
}

pub struct UserRepositoryImpl {
    pool: Arc<Mutex<DatabaseConnection>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<Mutex<DatabaseConnection>>) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn add_user(&self, user: &User, password: &String) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        println!("Adding users: {:?}", user);
        let statement = conn
            .client
            .prepare("INSERT INTO users (id, email, password, name) VALUES ($1, $2, $3, $4)");
        println!("Statement: {:?}", statement);
        let statement = match statement  {
            Ok(statement) => statement,
            Err(e) => {
                println!("Error preparing statement: {:?}", e);
                return Err(Box::new(e));
            }
        };
        println!("Statement: {:?}", statement);
        conn.client
            .execute(&statement, &[&user.id, &user.email, &password, &user.name])?;
        Ok(())
    }

    fn get_user(&self, id: &String) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        println!("Getting user: {:?}", id);
        let rows = conn.client.query("SELECT id, name, email FROM users", &[])?;

        println!("Rows: {:?}", rows);
        let statement = conn
            .client
            .prepare("SELECT id, name, email FROM users WHERE id = $1")
            ?;
        let rows = conn.client.query(&statement, &[&id])?;
        println!("Rows: {:?}", rows);
        println!("Rows: {:?}", rows.len());
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.get(0).unwrap();
        Ok(Some(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }))
    }

    fn get_user_by_email(&self, email: &String) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        let statement = conn
            .client
            .prepare("SELECT id, name, email FROM users WHERE email = $1")
            ?;
        let rows = conn.client.query(&statement, &[&email])?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.get(0).unwrap();
        Ok(Some(User {
            id: row.get(0),
            email: row.get(2),
            name: row.get(1),
        }))
    }

    fn get_users(&self, offset: u32, size: u32) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        let statement = conn
            .client
            .prepare("SELECT id, name, email FROM users LIMIT $1 OFFSET $2")
            ?;
        let rows = conn.client.query(&statement, &[&size, &offset])?;
        Ok(rows
            .iter()
            .map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            })
            .collect())
    }

    fn update_user(
        &self,
        user: User,
        password: Option<&String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        let statement = match password {
            Some(_) => {
                conn.client
                    .prepare("UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4")
                    ?
            }
            None => {
                conn.client
                    .prepare("UPDATE users SET name = $1, email = $2 WHERE id = $3")
                    ?
            }
        };
        match password {
            Some(password) => {
                conn.client
                    .execute(&statement, &[&user.name, &user.email, &password, &user.id])
                    ?;
            }
            None => {
                conn.client
                    .execute(&statement, &[&user.name, &user.email, &user.id])
                    ?;
            }
        }
        Ok(())
    }

    fn get_hashed_password(&self, email: &String) -> Result<Option<String>, Box<dyn Error>> {
        let mut conn = self.pool.lock().unwrap();
        let statement = conn
            .client
            .prepare("SELECT password FROM users WHERE email = $1")
            ?;
        let rows = conn.client.query(&statement, &[&email])?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.get(0).unwrap();
        Ok(Some(row.get(0)))
    }
}
