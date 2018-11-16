extern crate hyper;
extern crate serde_json;

use self::hyper::{Body, Request, Response, StatusCode};
use controller::*;
use {Error, Pool};

pub fn get_todos(_: Request<Body>, pool: &Pool) -> Result<Response<Body>, Error> {
    let conn = pool.get()?;
    let todos = list_todos(&conn)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&todos)?))?)
}
