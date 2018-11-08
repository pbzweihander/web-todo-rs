extern crate stdweb;
extern crate yew;

use yew::prelude::*;
use Todo;

pub struct TodoItem {
    todo: Todo,
    remove: Option<Callback<usize>>,
    toggle_complete: Option<Callback<usize>>,
}

pub enum TodoItemMessage {
    ToggleComplete,
    Remove,
}

#[derive(Clone, Default, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub remove: Option<Callback<usize>>,
    pub toggle_complete: Option<Callback<usize>>,
}

impl<CTX: 'static> Component<CTX> for TodoItem {
    type Message = TodoItemMessage;
    type Properties = TodoItemProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        TodoItem {
            todo: props.todo,
            remove: props.remove,
            toggle_complete: props.toggle_complete,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            TodoItemMessage::Remove => {
                if let Some(ref remove) = self.remove {
                    remove.emit(self.todo.id);
                }
            }
            TodoItemMessage::ToggleComplete => {
                if let Some(ref toggle_complete) = self.toggle_complete {
                    toggle_complete.emit(self.todo.id);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.todo = props.todo;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for TodoItem {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <div class="view",>
                <input
                    class="toggle",
                    type="checkbox",
                    onclick=|_| TodoItemMessage::ToggleComplete,
                    checked={ self.todo.done },
                >
                <label>{ &self.todo.content }</label>
                <button
                    class="destroy",
                    onclick=|_| TodoItemMessage::Remove,
                >
                </button>
            </div>
        )
    }
}
