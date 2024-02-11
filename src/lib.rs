#[macro_use]
extern crate rocket;
extern crate diesel;

use database::Db;
use rocket::launch;
use rocket::{Rocket, Build};
use dotenv::dotenv;
use rocket_db_pools::Database;


pub mod auth;
pub mod schema;
pub mod config;
pub mod database;
pub mod models;
pub mod routes;

//https://api.rocket.rs/v0.5-rc/rocket_db_pools/derive.Database.html
//https://api.rocket.rs/v0.5-rc/rocket_db_pools/diesel/index.html
// https://api.rocket.rs/v0.5-rc/rocket_db_pools/index.html
// https://docs.rs/tokio-postgres/0.7.6/tokio_postgres/config/struct.Config.html#url
//https://stackoverflow.com/questions/68633531/imlementing-connection-pooling-in-a-rust-diesel-app-with-r2d2

#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();
    rocket::custom(config::from_env())
    .attach(Db::init())        
    .mount(
            "/api/",
            routes![
                routes::hello,
                routes::auth::registration,
                routes::auth::get_users,
                routes::auth::login,
                routes::auth::get_account_info
            ]
        )
    .register("/", catchers![routes::not_found])        
    .attach(config::AppState::manage())
}