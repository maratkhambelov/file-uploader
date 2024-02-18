use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async move {
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            rocket.manage(AppState {
                secret: secret.into_bytes(),
            })
        })
    }
}

pub fn from_env() -> Figment {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("address", address))
        .merge(("databases", databases))
}
