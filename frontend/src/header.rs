extern crate stdweb;
extern crate yew;

use yew::prelude::*;

pub struct TodoHeader {
    new_content: String,
    add: Option<Callback<String>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct TodoHeaderProps {
    pub add: Option<Callback<String>>,
}

pub enum TodoHeaderMessage {
    OnInput(String),
    Add,
    None,
}

impl<CTX: 'static> Component<CTX> for TodoHeader {
    type Message = TodoHeaderMessage;
    type Properties = TodoHeaderProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        TodoHeader {
            new_content: String::new(),
            add: props.add,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            TodoHeaderMessage::OnInput(v) => {
                self.new_content = v;
            }
            TodoHeaderMessage::Add => {
                if let Some(ref add) = self.add {
                    add.emit(self.new_content.clone());
                }
            }
            _ => {}
        }
        false
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for TodoHeader {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <header class="header",>
                <h1>{ "Todos" }</h1>
                <input
                    class="new-todo",
                    placeholder="What needs to be done?",
                    oninput=|e| TodoHeaderMessage::OnInput(e.value),
                    onkeypress=|e| {
                        if e.key() == "Enter" {
                            TodoHeaderMessage::Add
                        } else {
                            TodoHeaderMessage::None
                        }
                    },
                />
            </header>
        )
    }
}
