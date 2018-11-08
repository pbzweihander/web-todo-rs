extern crate web_todo_frontend;
extern crate yew;

use web_todo_frontend::App as Frontend;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<(), Frontend>::new(()).mount_to_body();
    yew::run_loop();
}
