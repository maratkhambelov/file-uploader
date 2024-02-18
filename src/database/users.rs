use crate::schema::users::{self, table};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_db_pools::diesel::*;
use rocket_db_pools::Connection;
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, PartialEq)] //be careful with Deserialize, Serialize
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub secret: String,
}
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}
pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

pub fn create(conn: &mut PgConnection, username: &str, secret: &str) -> Result<User, Error> {
    let new_user = &NewUser { username, secret };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
}

pub fn login(username: &str, password: &str, conn: &mut PgConnection) -> Option<User> {
    let user = users::table
        .filter(users::username.eq(username))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let parsed_hash = PasswordHash::new(&user.secret).unwrap();
    let password_matches = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .is_ok();

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            username
        );
        None
    }
}
