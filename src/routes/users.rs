use crate::auth::GuardedRequest;
use crate::database::{Db, users::User};
use crate::schema::users;
use rocket_db_pools::Connection;  
use rocket_db_pools::diesel::*;

use rocket::serde::json::Json;
use rocket::http::Status;


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