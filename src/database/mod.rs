use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("diesel_postgres")]
pub struct Db(PgPool);

pub mod users;

pub mod file_results;
