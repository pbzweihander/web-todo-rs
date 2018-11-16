extern crate diesel;
extern crate dotenv;

use self::dotenv::dotenv;
use std::env;
use {ConnMan, Pool};

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnMan::new(database_url);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Error connecting to DB")
}
