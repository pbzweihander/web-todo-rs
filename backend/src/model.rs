use schema::todos;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub content: &'a str,
    pub done: bool,
}
