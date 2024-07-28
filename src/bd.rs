use postgres::{Client, NoTls};
use std::error::Error;

pub struct DatabaseConnection {
    pub client: Client,
}

impl DatabaseConnection {
    pub fn new() -> Result<DatabaseConnection, Box<dyn Error>> {
        let client = DatabaseConnection::create_connection()?;

        Ok(Self {
            client: client,
        })
    }

    fn create_connection() -> Result<Client, Box<dyn Error>> {
        // Connect to the database.
        let client= Client::connect(
            "host=db user=example password=example dbname=example",
            NoTls,
        )?;
        println!("Connected to the database");

        println!("Connected to the database");

        Ok(client)
    }
}
