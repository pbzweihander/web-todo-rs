#[derive(Clone, Default, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub content: String,
    pub done: bool,
}
