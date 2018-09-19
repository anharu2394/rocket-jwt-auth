use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(useres::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        useres::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<user> {
        useres::table.order(users::id.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(useres::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(useres::table.find(id)).execute(connection).is_ok()
    }
}i
