mod user;

use postgres::{Client, NoTls};
use user::User;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new("Lucas", "email");
    let id = Uuid::new_v4();
    let mut client = Client::connect(
        "host=db user=example password=example dbname=example",
        NoTls,
    )?;
    client.execute(
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
        &[&id.to_string(), &user.name(), &user.email()],
    )?;

    println!("User created with id: {}", id);

    Ok(())
}
