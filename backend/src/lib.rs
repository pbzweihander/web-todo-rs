#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate failure;

pub type Conn = diesel::sqlite::SqliteConnection;
pub type Error = failure::Error;

pub mod controller;
pub mod db;
pub mod model;
pub mod schema;
