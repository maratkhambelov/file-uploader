use crate::auth::generate_token;
use crate::config::AppState;
use crate::database::users::{NewUser, User};
use crate::database::Db;
use crate::schema::users;
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::State;
use rocket_db_pools::diesel::*;
use rocket_db_pools::Connection;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
struct RegistrationCredentials {
    username: String,
    password: String,
}

#[post("/registration", format = "json", data = "<credentials>")]
pub async fn registration(credentials: Json<RegistrationCredentials>, mut db: Connection<Db>) {
    let credentials_data = credentials.into_inner();

    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(credentials_data.password.as_bytes(), &salt)
        .expect("hash error")
        .to_string()
        .to_owned();

    let new_user: NewUser<'_> = NewUser {
        username: &credentials_data.username,
        secret: &hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut db)
        .await
        .expect("Failed to insert new user");
}

#[derive(Deserialize, Clone)]
struct LoginUserData {
    username: String,
    password: String,
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let user_json = serde_json::to_string(&self).map_err(|_| Status::InternalServerError)?;
        Ok(rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(Status::Ok)
            .sized_body(user_json.len(), Cursor::new(user_json))
            .finalize())
    }
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<LoginUserData>,
    mut db: Connection<Db>,
    state: &State<AppState>,
) -> Option<String> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let user: Option<User> = match users::table
        .filter(users::username.eq(&username))
        .get_result::<User>(&mut db)
        .map_err(|err| {
            eprintln!("Error getting user: {}", err);
        })
        .await
    {
        Ok(user) => Some(user),
        Err(_) => None,
    };

    let user = match user {
        Some(user) => user,
        None => return None,
    };

    let parsed_hash = PasswordHash::new(&user.secret).unwrap();
    let password_matches = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .is_ok();

    let secret = state.secret.clone();

    if password_matches {
        Some(generate_token(&user.id, &user.username, &secret))
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            username
        );
        None
    }
}
