extern crate stdweb;
extern crate yew;

use yew::prelude::*;
use {Todo, TodoFooter, TodoHeader, TodoList};

pub struct App {
    todos: Vec<Todo>,
    i: usize,
}

pub enum AppMessage {
    Add(String),
    ToggleComplete(usize),
    Remove(usize),
}

impl<CTX: 'static> Component<CTX> for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        App {
            todos: vec![],
            i: 0,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            AppMessage::Add(new_content) => {
                self.todos.push(Todo {
                    id: self.i,
                    content: new_content,
                    done: false,
                });
                self.i += 1;
            }
            AppMessage::Remove(id) => {
                self.todos.retain(|todo| todo.id != id);
            }
            AppMessage::ToggleComplete(id) => {
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

impl<CTX: 'static> Renderable<CTX, Self> for App {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <section class="todoapp",>
                <TodoHeader:
                    add=|new_content| AppMessage::Add(new_content),
                />
                <TodoList:
                    todos=self.todos.clone(),
                    toggle_complete=|id| AppMessage::ToggleComplete(id),
                    remove=|id| AppMessage::Remove(id),
                />
                <TodoFooter:
                    todo_count=self.todos.len(),
                />
            </section>
        )
    }
}
