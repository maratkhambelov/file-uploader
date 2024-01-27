#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::launch;
use rocket::{Rocket, Build};
use dotenv::dotenv;

pub mod schema;
pub mod config;
pub mod database;
pub mod models;
pub mod routes;


#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::auth::registration,
            ],
        )
        // .register("/", catchers![routes::not_found])
        // .attach(database::Db::fairing())
        // .attach(config::AppState::manage())
}