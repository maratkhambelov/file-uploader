#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::launch;
use rocket::{Rocket, Build};
use dotenv::dotenv;

mod config;
mod database;
mod models;
mod routes;
mod schema;


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