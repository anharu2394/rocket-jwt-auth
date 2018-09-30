#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
mod schema;
mod models;

use rocket_contrib::{Json, Value};
use models::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<user>")]
fn create(user: Json<NewUser>, connection: db::Connection) -> Json<User> {
    let insert = NewUser { ..user.into_inner() };
    Json(NewUser::create(insert, &connection))
}
#[get("/users")]
fn get_users(connection: db::Connection) -> Json<Vec<User>> {
    Json(User::read(&connection))
}

fn main() {
    rocket::ignite().mount("/", routes![index, create,get_users]).manage(db::connect()).launch();
}
