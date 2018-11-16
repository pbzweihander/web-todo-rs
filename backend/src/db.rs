extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use self::dotenv::dotenv;
use std::env;
use Conn;

pub fn establish_connection() -> Conn {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Conn::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
