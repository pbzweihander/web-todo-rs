extern crate hyper;
extern crate web_todo_backend;

use hyper::rt::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use web_todo_backend::db::establish_connection;
use web_todo_backend::router::route;

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();
    let pool = establish_connection();

    let new_service = move || {
        let pool = pool.clone();
        service_fn_ok(move |req| route(req, pool.clone()))
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("{}", e));

    println!("Server is running on {}!", addr);
    hyper::rt::run(server);
}
