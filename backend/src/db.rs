extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use self::diesel::sqlite::SqliteConnection;
use self::dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
