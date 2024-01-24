#[macro_use] extern crate rocket; //TODO: utilise "use" rather than "macro use"
//https://rocket.rs/v0.5/guide/upgrading/#configuration

use dotenv::dotenv;
use rocket::{Build, Rocket};

mod config;
mod routes;


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount("/", routes![hello])
        .mount(
            "/api",
            routes![
                routes::auth::registration,
            ],
        )
        .launch()
        .await;
}




