pub mod users;

// #[diesel(database("diesel_postgres"))]
pub struct Db(diesel::PgConnection);

