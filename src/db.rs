use std::env;

use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("/data/data.db".to_string());
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
