use diesel::prelude::*;
// use crate::auth::Auth;
// use crate::config::AppState;
use crate::database::{ Db}; //self,

use rocket::serde::json::{Json, Value, Error};
// use rocket::State;
// use serde::Deserialize;

use crate::database::users::NewUser;



// #[post("/registration", format = "json", data = "<new_user>")]
// pub async fn registration(new_user: Json<NewUser<'_>>, db: Db) -> Result<Value, Error<'_>> {
//     let new_user_data = new_user.into_inner();
//     let new_user = NewUser { username: new_user_data.username, secret: new_user_data.secret };
//
//     // db.run(move |conn| {
//     //     database::users::
//     // })
// }