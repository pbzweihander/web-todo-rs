extern crate diesel;
extern crate web_todo_backend;

use diesel::prelude::*;
use web_todo_backend::db::establish_connection;
use web_todo_backend::model::Todo;

fn main() {
    use web_todo_backend::schema::todos::dsl::*;

    let pool = establish_connection();

    let connection = pool.get().unwrap();
    let results = todos
        .load::<Todo>(&connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!(
            " {:>2} [{}] {}",
            todo.id,
            if todo.done { "x" } else { " " },
            todo.content
        );
    }
}
