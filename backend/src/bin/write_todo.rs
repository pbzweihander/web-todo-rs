extern crate diesel;
extern crate web_todo_backend;

use std::io::stdin;
use web_todo_backend::controller::create_todo;
use web_todo_backend::db::establish_connection;

fn main() {
    let connection = establish_connection();

    println!("New todo:");
    let mut content = String::new();
    stdin().read_line(&mut content).unwrap();
    let content = content.trim();

    let _ = create_todo(&connection, content.trim()).expect("Error saving todo");
    println!("\nSaved todo \"{}\"", content);
}
