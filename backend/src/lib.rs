#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

pub type Conn = diesel::pg::PgConnection;
pub type ConnMan = diesel::r2d2::ConnectionManager<Conn>;
pub type Pool = diesel::r2d2::Pool<ConnMan>;
pub type Error = failure::Error;

pub mod controller;
pub mod db;
pub mod model;
pub mod router;
pub mod schema;
pub mod view;
