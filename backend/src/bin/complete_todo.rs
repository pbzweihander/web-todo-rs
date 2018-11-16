extern crate web_todo_backend;

use std::env::args;
use web_todo_backend::controller::complete_todo;
use web_todo_backend::db::establish_connection;

fn main() {
    let mut args = args();
    let program = args.next().unwrap();
    let tid = args
        .next()
        .expect(&format!("Usage: {} {{id}}", program))
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = complete_todo(&connection, tid).expect(&format!("Unable to find todo {}", tid));
    println!("Todo {} completed!", tid);
}
