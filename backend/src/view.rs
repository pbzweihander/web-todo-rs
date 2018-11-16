extern crate hyper;

use self::hyper::{Body, Request, Response};
use controller::*;

fn get_todos(req: Request<Body>) -> Response<Body> {
    Response::new(Body::empty())
}
