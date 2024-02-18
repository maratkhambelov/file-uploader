use file_uploader;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    file_uploader::rocket().launch().await;

    Ok(())
}


