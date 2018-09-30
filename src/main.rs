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
mod encrypt;

use rocket_contrib::{Json, Value};
use models::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<user>")]
fn create(user: Json<NewUser>, conn: db::Connection) -> Json<User> {
    let insert = NewUser { ..user.into_inner() };
    Json(NewUser::create(insert, &conn))
}

#[get("/login")]
fn login(conn: db::Connection) {

}

#[get("/users")]
fn get_users(conn: db::Connection) -> Json<Vec<User>> {
    Json(User::read(&conn))
}

fn main() {
    rocket::ignite().mount("/", routes![index, create,get_users]).manage(db::connect()).launch();
}
