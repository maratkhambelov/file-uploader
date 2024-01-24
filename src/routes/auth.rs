use diesel::prelude::*;
// use crate::auth::Auth;
// use crate::config::AppState;
// use crate::database::{self, users::UserCreationError, Db};
// use crate::errors::{Errors, FieldValidator};

use rocket::serde::json::{Json, Value};
use rocket::State;
use serde::Deserialize;

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}


#[post("/registration", format = "json", data = "<new_user>")]
pub async fn registration(new_user: Json<NewUser>, db: database::Conn) -> Result<Value, Errors> {
    let new_user_data = new_user.into_inner();
    let new_user = NewUser { username: new_user_data.username, secret: new_user_data.secret };
}