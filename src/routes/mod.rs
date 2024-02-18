
pub mod auth;
pub mod file;
pub mod users;

#[catch(404)]
pub fn not_found() -> &'static str {
    "Route was not found!"
}

