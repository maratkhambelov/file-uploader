// #[macro_use] extern crate rocket; //TODO: utilise "use" rather than "macro use"
// //https://rocket.rs/v0.5/guide/upgrading/#configuration
//
// use dotenv::dotenv;
// use rocket::{Build, Rocket};
//
// mod config;
//
//
// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }
//
// #[rocket::main]
// async fn main() {
//     dotenv().ok();
//
//
//     rocket::custom(config::from_env())
//         .mount("/", routes![hello])
//         // .attach(database::Db::fairing())
//         .launch()
//         .await;
//     // rocket::ignite().mount("/", routes![hello]).launch();
// }


// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     //realworld::
//     lib::rocket_start().launch().await
// }


// mod routes;
// pub mod handlers;
// pub mod database;
// mod schema;



// #[rocket::main]
// #[launch]
// fn main() -> _ {
//     dotenv().ok();
//     rocket::custom(config::from_env()).mount("/", routes![hello]).launch();
//     // rocket::build().attach(database::init()).mount("/", routes![hello])
// }

// fn main() {
//     rocket::custom(config::get_env())
//         // .manage()
//         // .manage_database()
//         // .mount_timesheet_routes()
//         .launch();
// }


//use std::collections::HashMap;
// use std::env;
//
// use dotenv::dotenv;
// use rocket::Config;
// use serde_json::Value;

//mod config;
// pub fn get_env() -> Config {
//     dotenv().ok();
//
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
//     database_config.insert("url", Value::from(database_url));
//     databases.insert("diesel_postgres_pool", Value::from(database_config));
//
//     Config::build(rocket::Config::)
//         .port(port)
//         .extra("databases", databases)
//         .finalize()
//         .unwrap()
// }



// use dotenv::dotenv;
// use rocket::{get, launch};
//
//
// #[rocket::get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }
// #[rocket::launch]
// pub fn rocket() -> _ {
//     dotenv().ok();
//     // rocket::custom(config::from_env()).mount("/", routes![hello])
// }
