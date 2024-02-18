use diesel::prelude::*;
use crate::database::Db;
use rocket_db_pools::Connection;  
use rocket_db_pools::diesel::*;
use serde::{Deserialize, Serialize};

// #[derive(Insertable, Queryable, Deserialize, Debug, PartialEq)]
// #[diesel(table_name = crate::schema::file_results)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct FileResult<'a> {
//     pub id: &'a str,
//     pub result: &'a str,
// }
#[derive(Insertable, Queryable, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::file_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FileResult {
    pub id: String,
    pub result: String,
}


#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, PartialEq)] //be careful with Deserialize, Serialize
#[diesel(table_name = crate::schema::file_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct SelectedFileResult {
    pub id: String,
    pub result: String,
}

// pub struct NewFileResult<'a> {
//     pub id: &'a str,
//     pub result: &'a str,
// }


// #[derive(Queryable, Selectable, Deserialize, Serialize, Debug, PartialEq)] //be careful with Deserialize, Serialize
// #[diesel(table_name = crate::schema::users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
