pub mod models;
pub mod schema;


use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dotenv::dotenv;
use crate::models::{NewUser, User};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &mut PgConnection, username: &str, secret: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { username, secret };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
