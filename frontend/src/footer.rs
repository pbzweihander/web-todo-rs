extern crate stdweb;
extern crate yew;

use yew::prelude::*;

pub struct TodoFooter {
    todo_count: usize,
}

#[derive(Clone, Default, PartialEq)]
pub struct TodoFooterProps {
    pub todo_count: usize,
}

impl<CTX: 'static> Component<CTX> for TodoFooter {
    type Message = ();
    type Properties = TodoFooterProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        TodoFooter {
            todo_count: props.todo_count,
        }
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.todo_count = props.todo_count;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for TodoFooter {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <footer class="footer",>
                <span class="todo-count",>
                    <strong>{ self.todo_count }</strong>
                    {
                        format!(" {} left",
                            if self.todo_count == 1 {
                                "item"
                            } else {
                                "items"
                            }
                        )
                    }
                </span>
            </footer>
        )
    }
}
