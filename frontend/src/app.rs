extern crate stdweb;
extern crate yew;

use yew::prelude::*;
use {Todo, TodoFooter, TodoHeader, TodoList};

pub struct TodoApp {
    todos: Vec<Todo>,
    i: usize,
}

pub enum TodoAppMessage {
    Add(String),
    ToggleComplete(usize),
    Remove(usize),
}

impl<CTX: 'static> Component<CTX> for TodoApp {
    type Message = TodoAppMessage;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        TodoApp {
            todos: vec![],
            i: 0,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            TodoAppMessage::Add(new_content) => {
                self.todos.push(Todo {
                    id: self.i,
                    content: new_content,
                    done: false,
                });
                self.i += 1;
            }
            TodoAppMessage::Remove(id) => {
                self.todos.retain(|todo| todo.id != id);
            }
            TodoAppMessage::ToggleComplete(id) => {
                let i = self
                    .todos
                    .iter()
                    .position(|ref todo| todo.id == id)
                    .unwrap();
                let content = self.todos[i].content.clone();
                let done = self.todos[i].done;
                self.todos[i] = Todo { id, content, done };
            }
        }
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for TodoApp {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <section class="todoapp",>
                <TodoHeader:
                    add=|new_content| TodoAppMessage::Add(new_content),
                />
                <TodoList:
                    todos=self.todos.clone(),
                    toggle_complete=|id| TodoAppMessage::ToggleComplete(id),
                    remove=|id| TodoAppMessage::Remove(id),
                />
                <TodoFooter:
                    todo_count=self.todos.len(),
                />
            </section>
        )
    }
}
