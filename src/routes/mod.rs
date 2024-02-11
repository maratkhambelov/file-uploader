
pub mod auth;

#[catch(404)]
pub fn not_found() -> &'static str {
    "Resource was not found!"
}


#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}
