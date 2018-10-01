extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use encrypt;
use models::*;
use rocket::{State};
use db;
use db::Pool;
pub struct Token(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str, conn: db::Connection) -> bool {
    let decrypted_str: &str = &String::from_utf8(encrypt::decrypt(&key.to_string()).unwrap()).unwrap();
    let login_user: LoginUser = serde_json::from_str(decrypted_str).unwrap();
    println!("{:#?}", login_user);
    login_user.verify(&conn)
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token, ()> {
        let pool = request.guard::<State<Pool>>()?;
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if !is_valid(keys[0], db::Connection(pool.get().unwrap())) {
            return Outcome::Forward(());
        }

        return Outcome::Success(Token(key.to_string()));
    }
}
