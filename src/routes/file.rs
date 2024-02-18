use crate::auth::{GuardedRequest};
use crate::database::Db;
use crate::schema::{file_results };
use rocket::fs::TempFile;
use rocket_db_pools::Connection;  
use rocket_db_pools::diesel::*;
use serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use rocket::tokio::io::{AsyncReadExt, BufReader};
use rocket::http::Status;
use chrono::Utc;
use crate::database::file_results::{FileResult, SelectedFileResult};
// use std::path::Path;
// use rocket::form::FromForm;
// use rocket::http::ContentType;


// TODO:
// TODO ограничение на формат файла
// TODO add GuardedRequest
#[post("/upload", format = "text", data = "<file>")]
pub async fn upload(file: TempFile<'_>, guarded_response: GuardedRequest, mut db: Connection<Db>) -> Result<Json<Value>, Status> {
    
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

    // Генерируем уникальный ID на основе временной метки и имени файла
    let id = format!("{}_{}_{}", timestamp, file_name, guarded_response.user_id);
    let new_file_result: FileResult = FileResult { id: id.clone(), result: character_count.to_string() };
    // let new_file_result: FileResult<'_> = FileResult { id: &id, result: &character_count.to_string() };


    diesel::insert_into(file_results::table).values(&new_file_result).execute(&mut db).await.expect("Failed to insert new file_result");

    // Выполняем запрос для получения добавленной записи из базы данных
    let inserted_record = file_results::table.filter(file_results::id.eq(id))
                                            .first::<FileResult>(&mut db)
                                            .await
                                            .expect("Failed to fetch inserted file_result");

    // Возвращаем ответ с идентификатором и количеством символов
    Ok(Json(json!({ "id": inserted_record.id })))
}

  // diesel::insert_into(file_results::table).values(&new_file_result).execute(&mut db).await.expect("Failed to insert new file_result");
    // Ok(Json(json!({ "id": id  })))

//TODO: for all users
// 
// pub async fn file_result(id: &str, mut db: Connection<Db>) {
  
    
// //let file_result = 
//     match file_results::table
//     .filter(file_results::id.eq(id))
//     .get_result::<SelectedFileResult>(&mut db)
//     .map_err(|err| {
//         eprintln!("Error getting file_results: {}", err);
//         // Here you should return an appropriate error or handle it accordingly
//         // For example, you can return an HTTP response indicating failure.
//     })
//     .await {
//         Ok(file_result) => Some(file_result.result),
//         Err(_) => None,
//     };

// }


#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct FileResultRequestData {
    pub id: String,
}

#[get("/file_result", format = "json", data = "<requestData>")]
pub async fn file_result(requestData: Json<FileResultRequestData>, mut db: Connection<Db>) -> Option<String> {
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

    // let user: Option<FileResult> = match diesel::insert_into(file_results::table).values(&new_file_result)
    // .get_result::<FileResult>(&mut db)
    // .map_err(|err| {
    //     eprintln!("Error getting file_results: {}", err);
    //     // Here you should return an appropriate error or handle it accordingly
    //     // For example, you can return an HTTP response indicating failure.
    // }).await {
    //     Ok(file_result) => Some(file_result.result),
    //     Err(_) => None,
    // };