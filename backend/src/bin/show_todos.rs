extern crate diesel;
extern crate web_todo_backend;

use diesel::prelude::*;
use web_todo_backend::db::establish_connection;
use web_todo_backend::model::Todo;

fn main() {
    use web_todo_backend::schema::todos::dsl::*;

    let connection = establish_connection();
    let results = todos
        .limit(5)
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
