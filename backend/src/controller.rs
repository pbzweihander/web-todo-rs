extern crate diesel;

use diesel::prelude::*;
use model::NewTodo;
use {Conn, Error};

pub fn create_todo<'a>(conn: &Conn, content: &'a str) -> Result<usize, Error> {
    use schema::todos;

    let new_todo = NewTodo { content };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .map_err(Into::into)
}

pub fn complete_todo(conn: &Conn, tid: i32) -> Result<usize, Error> {
    use schema::todos::dsl::*;

    diesel::update(todos.find(tid))
        .set(done.eq(true))
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_todo(conn: &Conn, tid: i32) -> Result<usize, Error> {
    use schema::todos::dsl::*;

    diesel::delete(todos.find(tid))
        .execute(conn)
        .map_err(Into::into)
}
