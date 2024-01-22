// use rocket::config::Config;
// use rocket::figment::Figment;
// use std::collections::HashMap;
// use std::env;
//
// pub fn from_env() -> Figment {
//     let address = env::var("ADDRESS")
//         .unwrap_or_else(|_| "127.0.0.1".to_string());
//
//     let port = env::var("PORT")
//         .unwrap_or_else(|_| "8000".to_string())
//         .parse::<u16>()
//         .expect("PORT environment variable should parse to an integer");
//
//     let mut database_config = HashMap::new();
//     let mut databases = HashMap::new();
//     let database_url =
//         env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
//     database_config.insert("url", database_url);
//     databases.insert("diesel_postgres", database_config);
//
//     Config::figment()
//         .merge(("port", port))
//         .merge(("address", address))
//         .merge(("databases", databases))
// }


//use rocket::fairing::AdHoc;
// use std::env;
// use rocket_db_pools::{Database, Connection};
// use rocket_db_pools::diesel::{PgPool, prelude::*};
//
// // 1) https://api.rocket.rs/v0.5/rocket_db_pools/index.html
// // 2) https://api.rocket.rs/v0.5/rocket_db_pools/diesel/type.PgPool.html
//
// pub fn init() -> AdHoc {
//     AdHoc::on_ignite("Connecting to Database", |rocket| async {
//         match connect().await {
//             Ok(database) => rocket.manage(database),
//             Err(error) => {
//                 panic!("Cannot connect to instance:: {:?}", error)
//             }
//         }
//     })
// }
//
// async fn connect() -> { //mongodb::error::Result<Database>
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found.");
//     // let mongo_db_name = env::var("MONGO_DB_NAME").expect("DATABASE_NAME is not found.");
//
//     // let client_options = ClientOptions::parse(mongo_uri).await?;
//     // let client = Client::with_options(client_options)?;
//     // let database = client.database(mongo_db_name.as_str());
//
//     println!("DATABASE Connected!");
//
//     Ok() //DATABASE_URL
// }

//
// use rocket_db_pools::{Database};
// use rocket_db_pools::diesel;
//
// use diesel::prelude::*;

// #[derive(Database)]
// #[database("diesel_postgres")]
// pub struct Db(PgConnection);

