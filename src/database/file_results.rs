use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::file_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FileResult {
    pub id: String,
    pub result: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::file_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct SelectedFileResult {
    pub id: String,
    pub result: String,
}
