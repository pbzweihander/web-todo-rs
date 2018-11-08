extern crate stdweb;
extern crate yew;

use yew::prelude::*;
use Todo;
use TodoItem;

pub struct TodoList {
    todos: Vec<Todo>,
    remove: Option<Callback<usize>>,
    toggle_complete: Option<Callback<usize>>,
}

pub enum TodoListMessage {
    ToggleComplete(usize),
    Remove(usize),
}

#[derive(Clone, Default, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
    pub remove: Option<Callback<usize>>,
    pub toggle_complete: Option<Callback<usize>>,
}

impl TodoList {
    fn render_list<CTX: 'static>(todo: &Todo) -> Html<CTX, Self> {
        html! {
            <li class={ if todo.done { "completed" } else { "incompleted " } },>
                <TodoItem:
                    todo=todo.clone(),
                    toggle_complete=|todo| TodoListMessage::ToggleComplete(todo),
                    remove=|todo| TodoListMessage::Remove(todo),
                />
            </li>
        }
    }
}

impl<CTX: 'static> Component<CTX> for TodoList {
    type Message = TodoListMessage;
    type Properties = TodoListProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        TodoList {
            todos: props.todos,
            remove: props.remove,
            toggle_complete: props.toggle_complete,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            TodoListMessage::Remove(id) => {
                if let Some(ref remove) = self.remove {
                    remove.emit(id);
                }
            }
            TodoListMessage::ToggleComplete(id) => {
                if let Some(ref toggle_complete) = self.toggle_complete {
                    toggle_complete.emit(id);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.todos = props.todos;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for TodoList {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <section class="main",>
                <ul class="todo-list",>
                    { for self.todos.iter().map(Self::render_list) }
                </ul>
            </section>
        )
    }
}
