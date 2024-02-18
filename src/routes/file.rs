use crate::auth::GuardedRequest;
use crate::database::file_results::FileResult;
use crate::database::Db;
use crate::schema::file_results;
use chrono::Utc;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::tokio::io::{AsyncReadExt, BufReader};
use rocket_db_pools::diesel::*;
use rocket_db_pools::Connection;
use serde::Deserialize;

#[post("/upload", format = "text", data = "<file>")]
pub async fn upload(
    file: TempFile<'_>,
    guarded_response: GuardedRequest,
    mut db: Connection<Db>,
) -> Result<Json<Value>, Status> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file.open().await.map_err(|e| {
        error!("Failed to open file: {:?}", e);
        Status::InternalServerError
    })?);

    if let Err(e) = reader.take(1024000).read_to_end(&mut buffer).await {
        error!("Failed to read file: {:?}", e);
        return Err(Status::InternalServerError);
    }

    let timestamp = Utc::now().timestamp();

    let file_name: &str = file.name().unwrap_or("unknown_file");

    let contents: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

    let character_count = contents.chars().count();

    let id = format!("{}_{}_{}", timestamp, file_name, guarded_response.user_id);
    let new_file_result: FileResult = FileResult {
        id: id.clone(),
        result: character_count.to_string(),
    };

    diesel::insert_into(file_results::table)
        .values(&new_file_result)
        .execute(&mut db)
        .await
        .expect("Failed to insert new file_result");

    let inserted_record = file_results::table
        .filter(file_results::id.eq(id))
        .first::<FileResult>(&mut db)
        .await
        .expect("Failed to fetch inserted file_result");

    Ok(Json(json!({ "id": inserted_record.id })))
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct FileResultRequestData {
    pub id: String,
}

#[get("/file_result", format = "json", data = "<requestData>")]
pub async fn file_result(
    requestData: Json<FileResultRequestData>,
    mut db: Connection<Db>,
) -> Option<String> {
    match file_results::table
        .filter(file_results::id.eq(requestData.into_inner().id))
        .select(file_results::result)
        .first::<String>(&mut db)
        .await
    {
        Ok(result) => Some(result),
        Err(err) => {
            eprintln!("Error getting file_results: {}", err);
            None
        }
    }
}
