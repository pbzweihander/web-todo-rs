#[derive(Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub done: bool,
}
