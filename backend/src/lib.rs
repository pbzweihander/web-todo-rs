#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub type Conn = diesel::sqlite::SqliteConnection;

pub mod controller;
pub mod db;
pub mod model;
pub mod schema;
