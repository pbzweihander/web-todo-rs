extern crate hyper;

use self::hyper::{Body, Method, Request, Response, StatusCode};
use view::*;
use Pool;

#[derive(Debug, Fail)]
#[fail(display = "method {} not allowed on {}", _0, _1)]
struct MethodNotAllowedError(Method, String);

pub fn route(req: &Request<Body>, pool: &Pool) -> Response<Body> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/todo") => get_todos(req, pool),
        (m, u) => Err(MethodNotAllowedError(m.clone(), u.to_string()).into()),
    }.unwrap_or_else(|e| {
        eprintln!("{}", e);
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap()
    })
}
