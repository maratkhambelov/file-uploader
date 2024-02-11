use crate::auth::{generate_token, GuardedRequest};
use crate::database::Db;
use crate::models::users::User;
use crate::schema::users;
use rocket::futures::TryFutureExt;
use crate::database::users::NewUser;
use rocket_db_pools::Connection;  
use rocket_db_pools::diesel::*;
use serde::Deserialize;
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};
use rocket::serde::json::{json, Json, Value};
use rocket::response::Responder;
use std::io::Cursor;
use rocket::http::Status;

// password_hash::{
//     rand_core::OsRng,
//     PasswordHash, PasswordHasher, PasswordVerifier, SaltString
// },
// Scrypt
const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";




/**
* TODO:
* DONE 1) создать пользователя DONE
* DONE 2) - запустить route
* DONE 3) создать хеш при регистрации, занести в БД
* 4) при логине сравнивать хеш
* 5)  создавать токен при логине
6) возвращать токен при логине
7) в защищенном гварде проверять токен
8) создать CRUD по users
8) перенести в модуль логику по регистрации в database
*/

#[derive(Deserialize)]
struct RegistrationCredentials {
    username: String,
    password: String,
}

#[post("/registration", format = "json", data = "<credentials>")]
pub async fn registration(credentials: Json<RegistrationCredentials>, mut db: Connection<Db> )  {
    
    let credentials_data = credentials.into_inner();
    
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(credentials_data.password.as_bytes(), &salt)
        .expect("hash error")
        .to_string()
        .to_owned();
        
    let new_user: NewUser<'_> = NewUser { username: &credentials_data.username, secret: &hash };


    diesel::insert_into(users::table).values(&new_user).execute(&mut db).await.expect("Failed to insert new user");

}

// #[derive(Deserialize)]
// pub struct LoginUser {
//     user: LoginUserData,
// }

#[derive(Deserialize, Clone)]
struct LoginUserData {
    username: String,
    password: String,
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let user_json = serde_json::to_string(&self).map_err(|_| {
            rocket::http::Status::InternalServerError
        })?;
        Ok(rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::Ok)
            .sized_body(user_json.len(), Cursor::new(user_json))
            .finalize())
    }
}



//TODO:
/*
1) 
DONE возвращать пока ПУСТЫШКУ - COMMENT: я возвращаю сразу норм токен, ура!
DONE получить токен
DONE тправить в запрос токен
DONE обработать этот запрос с токеном
DONE возвращать или ошибку или выполнять запрос

2) создать CRUD по пользователю
3) создать запрос по загрузке файла
4) создать запрос получения результата

*/
//DONE - возвращать токен
// - добавить ошибку при несовпадении данных

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<LoginUserData>, mut db: Connection<Db>) -> Option<String>{
    
    let username = credentials.username.clone();
    let password = credentials.password.clone();
    
    let user = match users::table
        .filter(users::username.eq(&username))
        .get_result::<User>(&mut db)
        .map_err(|err| {
            eprintln!("Error getting user: {}", err);
            // Here you should return an appropriate error or handle it accordingly
            // For example, you can return an HTTP response indicating failure.
        })
        .await {
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

    if password_matches {
        Some(generate_token(&user.id, &user.username, SECRET.to_string().as_bytes()))
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            username
        );
        None
    }
}



//TODO: delete
#[get("/get_users")]
pub async fn get_users(mut db: Connection<Db>) -> Json<Vec<User>> {
    let all_users = users::table.load::<User>(&mut db).await.expect("Failed to fetch users");
    Json(all_users)
}

#[get("/get_account_info")]
pub async fn get_account_info(guarded_response: GuardedRequest, mut db: Connection<Db>) -> Result<Json<User>, Status> {
    use diesel::result::Error;

    match users::table.find(guarded_response.user_id).get_result::<User>(&mut db).await {
        Ok(user) => Ok(Json(user)),
        Err(Error::NotFound) => Err(Status::NotFound),
        Err(err) => {
            eprintln!("Error finding user: {}", err);
            Err(Status::InternalServerError)
        }
    }
}


