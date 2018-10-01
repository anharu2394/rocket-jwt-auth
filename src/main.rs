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
mod token;

use rocket_contrib::{Json};
use models::*;
use token::Token;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<user>")]
fn create(user: Json<NewUser>, conn: db::Connection) -> Json<User> {
    let insert = NewUser { ..user.into_inner() };
    Json(NewUser::create(insert, &conn))
}

#[post("/login", data="<login_user>")]
fn login(login_user: Json<LoginUser>, conn: db::Connection) -> String {
    match login_user.verify(&conn) {
        true => login_user.generate_token(),
        false => "Invalid".to_string(),
    }
}

#[get("/users")]
fn get_users(conn: db::Connection) -> Json<Vec<User>> {
    Json(User::read(&conn))
}

#[delete("/users/<id>")]
fn delete(id: i32, conn: db::Connection) -> String {
    match User::delete(id, &conn) {
        true => "OK".to_string(),
        false => "ERR".to_string(),
    }    
}

fn main() {
    rocket::ignite().mount("/", routes![index, create,get_users,login,delete]).manage(db::connect()).launch();
}
