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

#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();
    rocket::custom(config::from_env())
    .attach(Db::init())        
    .mount(
            "/api/",
            routes![
                routes::auth::registration,
                routes::users::get_users,
                routes::auth::login,
                routes::users::get_account_info,
                routes::file::upload,
                routes::file::file_result
            ]
        )
    .register("/", catchers![routes::not_found])        
    .attach(config::AppState::manage())
}