use diesel::prelude::*;
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
