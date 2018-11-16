extern crate diesel;

use diesel::prelude::*;
use model::{NewTodo, Todo};
use {Conn, Error};

pub fn create_todo<'a>(conn: &Conn, content: &'a str, done: bool) -> Result<usize, Error> {
    use schema::todos;

    let new_todo = NewTodo { content, done };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .map_err(Into::into)
}
