extern crate web_todo_frontend;
extern crate yew;

use web_todo_frontend::TodoApp;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<(), TodoApp>::new(()).mount_to_body();
    yew::run_loop();
}
