#[macro_use]
extern crate rocket;
use rocket::{
    serde::{json::Json, Serialize},
    Request,
};

mod planner;
mod programmer;
mod routes;
mod text_generation;

use routes::get_routes;

#[derive(Serialize)]
struct Error {
    error_msg: String,
    error_code: i32,
}

#[catch(404)]
fn not_found(req: &Request) -> Json<Error> {
    Json(Error {
        error_code: 404,
        error_msg: format!("I couldn't find '{}'. Try something else?", req.uri()),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/webhook", get_routes())
        .register("/", catchers![not_found])
}
