extern crate bcrypt;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::users;
use schema::users::dsl::{users as all_users};
use self::bcrypt::{DEFAULT_COST, hash, verify};
use encrypt;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn read(connection: &PgConnection) -> Vec<User> {
        all_users.order(users::id.desc()).load::<User>(connection).unwrap()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
    pub fn search(email: &String,connection: &PgConnection) -> String {
        all_users.filter(users::email.eq(email)).select(users::password).first(connection).unwrap()
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn create(user: NewUser, connection: &PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user.encrypt_pass())
            .execute(connection)
            .expect("Error creating new user");

        all_users.order(users::id.desc()).first(connection).unwrap()
    }
    fn encrypt_pass(&self) -> NewUser {
        NewUser { 
            name: self.name.clone(), 
            email: self.email.clone(),
            password: hash(&self.password, DEFAULT_COST).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl LoginUser {
    pub fn verify(&self, connection: &PgConnection) -> bool {
        verify(&self.password,&User::search(&self.email, connection)).unwrap()
    }
    pub fn generate_token(&self) -> String {
        encrypt::encrypt(&serde_json::to_string(&self.encrypt_pass()).unwrap())
    } 
    fn encrypt_pass(&self) -> LoginUser {
        LoginUser { 
            email: self.email.clone(),
            password: hash(&self.password, DEFAULT_COST).unwrap(),
        }
    }
}



