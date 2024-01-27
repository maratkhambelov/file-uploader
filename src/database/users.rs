use crate::models::users::User;
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}


// #[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}
pub fn create(
    conn: &PgConnection,
    username: &str,
    secret: &str,
) -> Result<User, UserCreationError> {
    // let salt = SaltString::generate(&mut OsRng);
    // let hash = Scrypt
    //     .hash_password(password.as_bytes(), &salt)
    //     .expect("hash error")
    //     .to_string()
    //     .to_owned();

    let new_user = &NewUser {
        username,
        secret,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}
