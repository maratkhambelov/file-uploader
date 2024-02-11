use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use serde::Deserialize;
use crate::models::users::User;
use crate::schema::users::{self, table};
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};
use crate::database::Db;
use rocket_db_pools::Connection;  
use rocket_db_pools::diesel::*;

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}

// impl From<Error> for UserCreationError {
//     fn from(err: Error) -> UserCreationError {
//         if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
//             match info.constraint_name() {
//                 Some("users_username_key") => return UserCreationError::DuplicatedUsername,
//                 Some("users_email_key") => return UserCreationError::DuplicatedEmail,
//                 _ => {}
//             }
//         }
//         panic!("Error creating user: {:?}", err)
//     }
// }


pub fn create(
    conn: &mut PgConnection,
    username: &str,
    secret: &str,
) -> Result<User, Error> { //UserCreationError

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
        // .map_err(Into::into)
}





pub fn login(username: &str, password: &str, conn: &mut PgConnection) -> Option<User> { // 
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


// pub fn login(username: &str, password: &str, conn: &mut Connection<Db>, ) -> Option<User> { // 
//     let user = users::table
//         .filter(users::username.eq(username))
//         .get_result::<User>(conn)
//         .map_err(|err| eprintln!("login_user: {}", err))
//         .ok()?;

//     let parsed_hash = PasswordHash::new(&user.secret).unwrap();
//     let password_matches = Scrypt
//         .verify_password(password.as_bytes(), &parsed_hash)
//         .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
//         .is_ok();

//     if password_matches {
//         Some(user)
//     } else {
//         eprintln!(
//             "login attempt for '{}' failed: password doesn't match",
//             username
//         );
//         None
//     }
// }